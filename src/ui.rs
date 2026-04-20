use std::collections::HashMap;

use bevy::{color::palettes::css::YELLOW, input_focus::InputFocus, prelude::*};
use chrono::Datelike;
use fluent::FluentArgs;
use fluent_datetime::{FluentDateTime, length};
use icu::{
    calendar::Date,
    time::{DateTime, Time},
};
use pyri_tooltip::{
    Tooltip, TooltipActivation, TooltipContent, TooltipDismissal, TooltipPlacement, TooltipTransfer,
};
use strum::IntoEnumIterator;

use crate::{
    bases::{BaseType, SpawnBaseEvent},
    constants::ui::{
        FONT_PATH, MENU_BACKGROUND, MENU_HOVER_BACKGROUND, MENU_PRESSED_BACKGROUND,
        UNICODE_FONT_PATH,
    },
    date::{GameDate, GameDateChangedEvent, GameSpeed},
    funds::{
        Expense, ExpenseCategory, Funds, FundsAmount, FundsChangedEvent, Income, IncomeCategory,
    },
    locales::{FluentBundleResource, TextKey},
    regions::{Region, RegionsReloadedEvent},
    state::{GameState, MainSetupSet},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Load), setup_fonts)
            .add_systems(
                OnEnter(GameState::Main),
                (setup, setup_map, setup_regions)
                    .chain()
                    .in_set(MainSetupSet::Ui),
            )
            .add_systems(Update, (update_button_colors, update_funds_displays));
    }
}

#[derive(Component)]
#[relationship(relationship_target = View)]
pub struct ViewOf(pub Entity);

#[derive(Component)]
#[relationship_target(relationship = ViewOf)]
pub struct View(Entity);

#[derive(Resource)]
struct FontHandle(Handle<Font>);

#[derive(Resource)]
struct UnicodeFontHandle(Handle<Font>);

#[derive(Component)]
struct MapUi;

#[derive(Component)]
struct GameDateUi;

#[derive(Component)]
struct FundsUi;

#[derive(Component)]
struct FundsTooltip;

#[derive(Component)]
#[require(Text)]
struct FundsDisplay(pub FundsAmount);

#[derive(Component)]
struct RegionUi;

fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandle(asset_server.load(FONT_PATH)));
    commands.insert_resource(UnicodeFontHandle(asset_server.load(UNICODE_FONT_PATH)));
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.insert_resource(InputFocus::default());
}

fn setup_map(
    mut commands: Commands,
    font_handle: Res<FontHandle>,
    unicode_font_handle: Res<UnicodeFontHandle>,
    asset_server: Res<AssetServer>,
) {
    let tooltip_content = commands
        .spawn((
            FundsTooltip,
            Node {
                flex_direction: FlexDirection::Column,
                border: UiRect::all(px(2)),
                padding: UiRect::all(px(3)),
                ..default()
            },
            BorderColor::all(YELLOW),
            BackgroundColor(MENU_BACKGROUND.into()),
            Visibility::Hidden,
            ZIndex(1),
        ))
        .id();

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: vw(100.0),
            height: vh(100.0),
            ..Default::default()
        })
        .with_children(|parent| {
            // Top status bar
            parent
                .spawn((
                    Node {
                        width: vw(100.0),
                        height: vh(5.0),
                        border: UiRect {
                            left: vw(0.0),
                            right: vw(0.0),
                            top: vh(0.5),
                            bottom: vh(0.5),
                        },
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    BackgroundColor(MENU_BACKGROUND.into()),
                ))
                .with_children(|parent| {
                    // Funds counter
                    parent.spawn((
                        Node {
                            padding: UiRect::right(px(5)),
                            ..default()
                        },
                        // will be updated by on_funds_changed
                        TextFont {
                            font: font_handle.0.clone(),
                            ..default()
                        },
                        TextKey::new_no_args("funds-display"),
                        FundsUi,
                        Tooltip {
                            content: TooltipContent::Custom(tooltip_content),
                            placement: TooltipPlacement::CURSOR,
                            activation: TooltipActivation::default(),
                            dismissal: TooltipDismissal {
                                // Not sure what units these are
                                on_distance: 400.0,
                                on_click: false,
                            },
                            transfer: TooltipTransfer::default(),
                        },
                    ));
                    // Game date display
                    parent.spawn((
                        Node {
                            padding: UiRect::right(px(5)),
                            ..default()
                        },
                        // will be updated by on_game_date_changed
                        TextFont {
                            font: font_handle.0.clone(),
                            ..default()
                        },
                        TextKey::new_no_args("game-date-display"),
                        GameDateUi,
                    ));
                    // Separate left-aligned and right-aligned status fields
                    parent.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });
                    parent.spawn((
                        Button,
                        GameSpeed(1.0),
                        // RIGHTWARDS ARROW
                        Text("\u{2192}".to_string()),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Button,
                        GameSpeed(2.0),
                        // RIGHTWARDS PAIRED ARROWS
                        Text("\u{21C9}".to_string()),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Button,
                        GameSpeed(5.0),
                        // THREE RIGHTWARDS ARROWS
                        Text("\u{21F6}".to_string()),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                    ));
                });
            parent.spawn((
                ImageNode {
                    image: asset_server.load("textures/earth_night.jpg"),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    width: vw(100.0),
                    height: vh(95.0),
                    ..default()
                },
                MapUi,
            ));
        });

    commands.add_observer(on_game_date_changed_funds_tooltip);
    commands.add_observer(on_funds_changed);
    commands.add_observer(on_game_date_changed);
    commands.trigger(FundsChangedEvent);
    commands.trigger(GameDateChangedEvent);
}

fn on_game_date_changed(
    _: On<GameDateChangedEvent>,
    date: Res<GameDate>,
    mut text: Single<(&mut Text, &TextKey), With<GameDateUi>>,
    bundle: Res<FluentBundleResource>,
) {
    // Do a little dance
    let date = Date::try_new_iso(date.0.year(), date.0.month() as u8, date.0.day() as u8).unwrap();
    let datetime = DateTime {
        date,
        time: Time::start_of_day(),
    };
    let mut datetime: FluentDateTime = datetime.into();
    datetime.options.set_date_style(Some(length::Date::Long));
    let mut args = FluentArgs::new();
    args.set("date", datetime);
    text.0.0 = text.1.get(&bundle, &args);
}

fn on_funds_changed(
    _: On<FundsChangedEvent>,
    funds: Res<Funds>,
    mut text: Single<(&mut Text, &TextKey), With<FundsUi>>,
    bundle: Res<FluentBundleResource>,
) {
    let mut args = FluentArgs::new();
    args.set("funds", funds.0);
    text.0.0 = text.1.get(&bundle, &args);
}

fn setup_regions(
    mut commands: Commands,
    map_ui: Single<Entity, With<MapUi>>,
    regions: Query<(Entity, &Region)>,
    font_handle: Res<FontHandle>,
    bundle: Res<FluentBundleResource>,
) {
    for (entity, region) in regions.iter() {
        let Region { name, settings } = region;

        commands
            .spawn((
                ChildOf(*map_ui),
                ViewOf(entity),
                Node {
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    left: percent(settings.x),
                    top: percent(settings.y),
                    ..default()
                },
                RegionUi,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            border: UiRect::all(px(2)),
                            border_radius: BorderRadius::all(px(10)),
                            padding: UiRect::all(px(10)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        BackgroundColor(MENU_BACKGROUND.into()),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextKey::new(format!("region-{name}"), &bundle),
                            TextFont {
                                font: font_handle.0.clone(),
                                ..default()
                            },
                        ));
                    });
            });
    }

    commands.add_observer(on_regions_reloaded);
    commands.add_observer(on_spawn_base);
}

// INFO: Assume only the settings have changed, while none is added or removed.
fn on_regions_reloaded(
    _: On<RegionsReloadedEvent>,
    regions: Query<&Region>,
    mut region_uis: Query<(&mut Node, &ViewOf), With<RegionUi>>,
) {
    for (mut node, region_ui) in region_uis.iter_mut() {
        if let Ok(region) = regions.get(region_ui.0) {
            node.left = percent(region.settings.x);
            node.top = percent(region.settings.y);
        }
    }
}

fn on_spawn_base(
    event: On<SpawnBaseEvent>,
    mut commands: Commands,
    regions: Query<(&Region, &View)>,
    base_types: Query<&BaseType>,
    font_handle: Res<FontHandle>,
    bundle: Res<FluentBundleResource>,
) {
    let region_ui = regions.get(event.region).unwrap().1.0;
    let base_type = base_types.get(event.base_type).unwrap();

    commands
        .spawn((
            ChildOf(region_ui),
            ViewOf(event.base_type),
            Node {
                border: UiRect::all(px(1)),
                padding: UiRect::horizontal(px(2)),
                justify_content: JustifyContent::End,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(MENU_BACKGROUND.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextKey::new(format!("basetype-{}", &base_type.name), &bundle),
                TextFont {
                    font: font_handle.0.clone(),
                    ..default()
                },
            ));
        });
}

fn update_funds_displays(
    mut q: Query<(&mut Text, &FundsDisplay), Changed<FundsDisplay>>,
    bundle: Res<FluentBundleResource>,
) {
    for (mut text, funds) in &mut q {
        let mut args = FluentArgs::new();
        args.set("funds", funds.0);
        text.0 = bundle.get("funds", Some(&args));
    }
}

fn update_button_colors(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                *background = MENU_PRESSED_BACKGROUND.into();
            }
            Interaction::Hovered => {
                *background = MENU_HOVER_BACKGROUND.into();
            }
            Interaction::None => {
                *background = MENU_BACKGROUND.into();
            }
        }
    }
}

fn on_game_date_changed_funds_tooltip(
    _: On<GameDateChangedEvent>,
    mut commands: Commands,
    incomes: Query<&Income>,
    expenses: Query<&Expense>,
    tooltip: Single<Entity, With<FundsTooltip>>,
    font_handle: Res<FontHandle>,
    bundle: Res<FluentBundleResource>,
) {
    fn income_expense_row(
        mut commands: Commands,
        parent: Entity,
        text_font: &TextFont,
        category: String,
        count: usize,
        funds: FundsAmount,
        bundle: &Res<FluentBundleResource>,
    ) {
        commands
            .spawn((
                // Node to represent the row
                Node::default(),
                ChildOf(parent),
            ))
            .with_children(|parent| {
                parent.spawn((Text::new(format!("{count}x ")), text_font.clone()));
                parent.spawn((TextKey::new(category, bundle), text_font.clone()));
                parent.spawn(Node {
                    flex_grow: 1.0,
                    padding: UiRect::left(px(5)),
                    ..default()
                });
                parent.spawn((FundsDisplay(funds), text_font.clone()));
            });
    }

    let tooltip = tooltip.entity();
    commands.entity(tooltip).despawn_children();

    // Completely refresh the tooltip contents
    let text_font = TextFont {
        font: font_handle.0.clone(),
        ..default()
    };
    let hrule = (
        Node {
            min_width: percent(80),
            min_height: px(1),
            margin: UiRect::vertical(px(5)),
            ..default()
        },
        BackgroundColor(YELLOW.into()),
        ChildOf(tooltip),
    );
    commands.spawn((
        TextKey::new("income-tooltip-header", &bundle),
        text_font.clone(),
        ChildOf(tooltip),
    ));
    commands.spawn(hrule.clone());

    let mut income_ledger: HashMap<IncomeCategory, (i64, usize)> = HashMap::default();
    for Income(amount, category) in incomes {
        let (funds, count) = income_ledger.entry(*category).or_default();
        *funds += amount;
        *count += 1
    }
    for category in IncomeCategory::iter() {
        if let Some((funds, count)) = income_ledger.get(&category) {
            let category: &str = category.into();
            let category = format!("income-category-{category}");
            income_expense_row(
                commands.reborrow(),
                tooltip,
                &text_font,
                category,
                *count,
                *funds,
                &bundle,
            );
        }
    }
    commands.spawn((
        TextKey::new("expense-tooltip-header", &bundle),
        text_font.clone(),
        ChildOf(tooltip),
    ));
    commands.spawn(hrule);
    let mut expense_ledger: HashMap<ExpenseCategory, (i64, usize)> = HashMap::default();
    for Expense(amount, category) in expenses {
        let (funds, count) = expense_ledger.entry(*category).or_default();
        *funds += amount;
        *count += 1
    }
    for category in ExpenseCategory::iter() {
        if let Some((funds, count)) = expense_ledger.get(&category) {
            let category: &str = category.into();
            let category = format!("expense-category-{category}");
            income_expense_row(
                commands.reborrow(),
                tooltip,
                &text_font,
                category,
                *count,
                *funds,
                &bundle,
            );
        }
    }
}
