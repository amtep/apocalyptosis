use std::collections::HashMap;

use bevy::{input_focus::InputFocus, prelude::*, window::WindowResized};
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
    bases::{Base, Basetype},
    constants::ui::*,
    followers::Follower,
    funds::{
        Expense, ExpenseCategory, Funds, FundsAmount, FundsChangedEvent, Income, IncomeCategory,
    },
    regions::{BasePlot, Location, Region},
    state::{GameState, MainSetupSet},
    text::{FluentBundleWrapper, TextKey},
    time::{
        CurrentGameSpeed, GameDate, GameDateChangedEvent, GameSpeed, GameSpeedAction,
        GameSpeedChangedEvent,
    },
};

mod buttons;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Load), setup_fonts)
        .init_resource::<UiScale>()
        .init_resource::<InputFocus>()
        .add_systems(Update, read_window_resized_messages)
        .add_systems(
            OnEnter(GameState::Main),
            (setup_map, setup_regions).chain().in_set(MainSetupSet::Ui),
        )
        .add_systems(
            Update,
            (
                (update_speed_buttons, update_funds_displays).run_if(in_state(GameState::Main)),
                buttons::update_button_backgrounds,
            ),
        )
        .add_systems(
            Update,
            update_game_speed_state
                .run_if(resource_changed::<CurrentGameSpeed>.and(in_state(GameState::Main))),
        );
}

#[derive(Component)]
#[relationship(relationship_target = Views)]
struct ViewOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ViewOf, linked_spawn)]
struct Views(Vec<Entity>);

#[derive(Resource)]
struct FontHandle(Handle<Font>);

#[derive(Resource)]
struct DisplayFontHandle(Handle<Font>);

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
struct FundsDisplay(FundsAmount);

#[derive(Component)]
struct RegionUi;

#[derive(Component)]
struct BasePlotUi;

#[derive(Component)]
struct BaseUi;

#[derive(Component)]
struct FollowerList;

fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FontHandle(asset_server.load(FONT_PATH)));
    commands.insert_resource(DisplayFontHandle(asset_server.load(FONT_DISPLAY_PATH)));
    commands.insert_resource(UnicodeFontHandle(asset_server.load(UNICODE_FONT_PATH)));
}

fn read_window_resized_messages(
    mut reader: MessageReader<WindowResized>,
    mut ui_scale: ResMut<UiScale>,
) {
    if let Some(WindowResized { height, .. }) = reader.read().last() {
        info!("window resized; height: {height}");
        ui_scale.0 = height / 720.0;
    }
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
            BackgroundColor::from(MENU_BACKGROUND),
            Visibility::Hidden,
            ZIndex(1),
        ))
        .id();

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: percent(100.0),
            height: percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        })
        .with_children(|parent| {
            // Top status bar
            parent
                .spawn((
                    Node {
                        width: percent(100.0),
                        border: UiRect::vertical(px(2)),
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(BORDER),
                    BackgroundColor::from(MENU_BACKGROUND),
                ))
                .with_children(|parent| {
                    // Funds counter
                    parent.spawn((
                        Node {
                            min_width: px(75),
                            ..default()
                        },
                        // will be updated by on_funds_changed
                        TextFont {
                            font: font_handle.0.clone(),
                            ..default()
                        },
                        FundsDisplay(0),
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
                        GameSpeedAction::TogglePause,
                        Node {
                            width: px(25),
                            ..Default::default()
                        }, // 23F8 DOUBLE VERTICAL BAR would be better but is not in the font.
                        // DOUBLE VERTICAL LINE
                        Text("\u{2016}".to_string()),
                        TextColor::from(TEXT),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                    parent.spawn((
                        Button,
                        GameSpeedAction::SetSpeed(GameSpeed::Normal),
                        Node {
                            width: px(25),
                            ..Default::default()
                        }, // RIGHTWARDS ARROW
                        Text("\u{2192}".to_string()),
                        TextColor::from(TEXT_HIGHLIGHT),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                    parent.spawn((
                        Button,
                        GameSpeedAction::SetSpeed(GameSpeed::Fast),
                        Node {
                            width: px(25),
                            ..Default::default()
                        }, // RIGHTWARDS PAIRED ARROWS
                        Text("\u{21C9}".to_string()),
                        TextColor::from(TEXT),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                    parent.spawn((
                        Button,
                        GameSpeedAction::SetSpeed(GameSpeed::Faster),
                        Node {
                            width: px(25),
                            ..Default::default()
                        }, // THREE RIGHTWARDS ARROWS
                        Text("\u{21F6}".to_string()),
                        TextColor::from(TEXT),
                        TextFont {
                            font: unicode_font_handle.0.clone(),
                            ..default()
                        },
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                });
            parent.spawn((
                ImageNode {
                    image: asset_server.load(TEXTURE_EARTH_BACKGROUND),
                    image_mode: NodeImageMode::Stretch,
                    ..default()
                },
                Node {
                    width: percent(100.0),
                    flex_grow: 1.0,
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
    bundle: Res<FluentBundleWrapper>,
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
    mut funds_display: Single<&mut FundsDisplay, With<FundsUi>>,
) {
    funds_display.0 = funds.0;
}

fn setup_regions(
    mut commands: Commands,
    map_ui: Single<Entity, With<MapUi>>,
    regions: Query<(Entity, &Region, &Location, &Children)>,
    base_plots: Query<&Location, With<BasePlot>>,
    font_handle: Res<DisplayFontHandle>,
    bundle: Res<FluentBundleWrapper>,
) {
    for (entity, region, location, children) in regions.iter() {
        commands
            .spawn((
                ChildOf(*map_ui),
                ViewOf(entity),
                Node {
                    position_type: PositionType::Absolute,
                    left: percent(location.x),
                    top: percent(location.y),
                    ..default()
                },
                UiTransform {
                    translation: Val2::percent(-50.0, -50.0),
                    ..Default::default()
                },
                RegionUi,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            border: UiRect::all(px(1)),
                            border_radius: BorderRadius::all(px(10)),
                            padding: UiRect::all(px(5)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        BorderColor::all(BORDER),
                        BackgroundColor::from(MENU_BACKGROUND),
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextKey::new(format!("region-{}", region.name), &bundle),
                            TextFont {
                                font: font_handle.0.clone(),
                                ..default()
                            },
                        ));
                    });
            });

        for child in children {
            let location = base_plots.get(*child).unwrap();
            commands.spawn((
                ChildOf(*map_ui),
                ViewOf(*child),
                BasePlotUi,
                Node {
                    left: percent(location.x),
                    top: percent(location.y),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                UiTransform {
                    translation: Val2::percent(-50.0, -50.0),
                    ..Default::default()
                },
            ));
        }
    }

    commands.add_observer(on_location_reloaded);
    commands.add_observer(on_spawn_base);
    commands.add_observer(on_changed_follower::<Insert>);
    commands.add_observer(on_changed_follower::<Replace>);
}

// INFO: Assume only the location has changed, while none is added or removed.
fn on_location_reloaded(
    event: On<Insert, Location>,
    parts: Query<(&Location, &Views)>,
    mut nodes: Query<&mut Node>,
) {
    let (location, views) = parts.get(event.entity).unwrap();

    for view in &views.0 {
        if let Ok(mut node) = nodes.get_mut(*view) {
            node.left = percent(location.x);
            node.top = percent(location.y);
        }
    }
}

fn on_spawn_base(
    event: On<Add, Base>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    base_plots: Query<&Views, With<BasePlot>>,
    base_plot_uis: Query<&BasePlotUi>,
    base_types: Query<&Basetype>,
    font_handle: Res<FontHandle>,
    bundle: Res<FluentBundleWrapper>,
) {
    let base_plot = parents.get(event.entity).unwrap().0;
    let base_plot_views = &base_plots.get(base_plot).unwrap().0;
    let base_plot_ui = base_plot_views
        .iter()
        .find(|view| base_plot_uis.contains(**view))
        .unwrap();
    let base_type = base_types.get(event.entity).unwrap();

    commands
        .spawn((
            ChildOf(*base_plot_ui),
            ViewOf(event.entity),
            BaseUi,
            Node {
                flex_direction: FlexDirection::Column,
                border: UiRect::all(px(1)),
                padding: UiRect::horizontal(px(2)),
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(WHITE),
            BackgroundColor::from(MENU_BACKGROUND),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextKey::new(format!("basetype-{}", &base_type.name), &bundle),
                TextFont {
                    font_size: 14.0,
                    font: font_handle.0.clone(),
                    ..default()
                },
            ));
            parent.spawn((
                FollowerList,
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
            ));
        });
}

fn on_changed_follower<E: EntityEvent>(
    event: On<E, Follower>,
    mut commands: Commands,
    parents: Query<&ChildOf>,
    children: Query<&Children>,
    followers: Query<&Follower>,
    base_views: Query<&Views, With<Base>>,
    base_uis: Query<&BaseUi>,
    follower_lists: Query<&FollowerList>,
    unicode_font_handle: Res<UnicodeFontHandle>,
) {
    let base = parents.get(event.event_target()).unwrap().0;
    let base_views = base_views.get(base).unwrap();
    let base_ui = base_views
        .iter()
        .find(|view| base_uis.contains(*view))
        .unwrap();
    let follower_list = children
        .get(base_ui)
        .unwrap()
        .iter()
        .find(|fl| follower_lists.contains(*fl))
        .unwrap();

    commands.entity(follower_list).despawn_children();

    let mut followers: Vec<Follower> = children
        .get(base)
        .unwrap()
        .iter()
        .map(|follower| *followers.get(follower).unwrap())
        .collect();

    followers.sort_unstable();

    let text_font = TextFont {
        font: unicode_font_handle.0.clone(),
        ..Default::default()
    };

    let bundles: Vec<_> = followers
        .iter()
        .map(|f| {
            let text = match f {
                Follower::Priest => Text::new("☉"),
                Follower::Goon => Text::new("♁"),
                Follower::Minion => Text::new("☿"),
            };
            (ChildOf(follower_list), text, text_font.clone())
        })
        .collect();

    commands.spawn_batch(bundles);
}

fn update_funds_displays(
    mut q: Query<(&mut Text, &FundsDisplay), Changed<FundsDisplay>>,
    bundle: Res<FluentBundleWrapper>,
) {
    for (mut text, funds) in &mut q {
        let mut args = FluentArgs::new();
        args.set("funds", funds.0);
        text.0 = bundle.get("funds", Some(&args));
    }
}

fn update_game_speed_state(
    current_game_speed: Res<CurrentGameSpeed>,
    mut game_speed_buttons: Query<(&mut TextColor, &GameSpeedAction)>,
) {
    for (mut text_color, &speed_action) in game_speed_buttons.iter_mut() {
        let is_active = speed_action == GameSpeedAction::TogglePause && current_game_speed.paused
            || speed_action == GameSpeedAction::SetSpeed(current_game_speed.speed)
                && !current_game_speed.paused;
        if is_active {
            *text_color = TEXT_HIGHLIGHT.into();
        } else {
            *text_color = TEXT.into();
        }
    }
}

fn update_speed_buttons(
    mut commands: Commands,
    mut input_focus: ResMut<InputFocus>,
    mut q: Query<(Entity, &Interaction, &mut Button, &GameSpeedAction), Changed<Interaction>>,
) {
    for (entity, interaction, mut button, game_speed_action) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                // alert the accessibility system
                button.set_changed();
                commands.trigger(GameSpeedChangedEvent(*game_speed_action));
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}

fn on_game_date_changed_funds_tooltip(
    _: On<FundsChangedEvent>,
    mut commands: Commands,
    incomes: Query<&Income>,
    expenses: Query<&Expense>,
    tooltip: Single<Entity, With<FundsTooltip>>,
    font_handle: Res<FontHandle>,
    bundle: Res<FluentBundleWrapper>,
) {
    fn income_expense_row(
        mut commands: Commands,
        parent: Entity,
        text_font: &TextFont,
        category: String,
        count: usize,
        funds: FundsAmount,
        bundle: &Res<FluentBundleWrapper>,
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
        font_size: 14.0,
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
        BackgroundColor::from(YELLOW),
        ChildOf(tooltip),
    );
    commands.spawn((
        TextKey::new("income-tooltip-header", &bundle),
        text_font.clone(),
        ChildOf(tooltip),
    ));
    commands.spawn(hrule.clone());

    let mut income_ledger: HashMap<IncomeCategory, (FundsAmount, usize)> = HashMap::default();
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
    let mut expense_ledger: HashMap<ExpenseCategory, (FundsAmount, usize)> = HashMap::default();
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
