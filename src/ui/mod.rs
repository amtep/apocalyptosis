use std::collections::HashMap;

use bevy::{input_focus::InputFocus, prelude::*, ui::UiSystems, window::WindowResized};
use pyri_tooltip::{
    Tooltip, TooltipActivation, TooltipContent, TooltipDismissal, TooltipPlacement, TooltipTransfer,
};
use strum::IntoEnumIterator;

use crate::{
    bases::{Base, Basetype},
    constants::ui::*,
    followers::Follower,
    funds::{Expense, ExpenseCategory, Funds, FundsAmount, Income, IncomeCategory},
    regions::{BasePlot, Location, Region},
    state::{GameState, MainSetupSet},
    suspicion::{IntelligenceSuspicion, MediaSuspicion, PoliceSuspicion, ScientificSuspicion},
    text::TextKey,
    time::{CurrentGameSpeed, GameDate, GameSpeed, GameSpeedAction, GameSpeedChangedEvent},
    ui::{
        buttons::setup_observe_buttons,
        main_menu::{CultName, CultSymbol, setup_main_menu},
    },
};

mod buttons;
mod dialog;
pub mod main_menu;
pub mod save_load;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Load), setup_fonts)
        .init_resource::<UiScale>()
        .init_resource::<InputFocus>()
        .add_systems(OnExit(GameState::Load), setup_observe_buttons)
        .add_systems(Update, read_window_resized_messages)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(
            OnEnter(GameState::Main),
            (setup_map, setup_regions).chain().in_set(MainSetupSet::Ui),
        )
        .add_systems(
            Update,
            update_regional_suspicion.run_if(in_state(GameState::Main)),
        )
        .add_systems(
            Update,
            update_game_date
                .run_if(resource_exists_and_changed::<GameDate>.and(in_state(GameState::Main))),
        )
        .add_systems(
            Update,
            (update_funds_tooltip, update_funds)
                .run_if(resource_exists_and_changed::<Funds>.and(in_state(GameState::Main))),
        )
        .add_systems(
            Update,
            update_suspicion.run_if(
                (resource_exists_and_changed::<IntelligenceSuspicion>
                    .or(resource_exists_and_changed::<ScientificSuspicion>))
                .and(in_state(GameState::Main)),
            ),
        )
        .add_systems(
            Update,
            update_game_speed_state.run_if(
                resource_exists_and_changed::<CurrentGameSpeed>.and(in_state(GameState::Main)),
            ),
        )
        .add_systems(
            PostUpdate,
            update_meter_display::<u32>
                .run_if(in_state(GameState::Main))
                .before(UiSystems::Prepare),
        );
}

#[derive(Component)]
#[relationship(relationship_target = Views)]
struct ViewOf(Entity);

#[derive(Component)]
#[relationship_target(relationship = ViewOf, linked_spawn)]
struct Views(Vec<Entity>);

#[derive(Resource)]
pub struct FontHandle(pub Handle<Font>);

#[derive(Resource)]
pub struct DisplayFontHandle(pub Handle<Font>);

#[derive(Resource)]
pub struct UnicodeFontHandle(pub Handle<Font>);

#[derive(Component)]
struct MapUi;

#[derive(Component)]
struct GameDateUi;

#[derive(Component)]
struct FundsUi;

#[derive(Component)]
struct FundsTooltip;

#[derive(Component)]
#[require(Text, TextColor)]
struct MeterDisplay<T: PartialOrd + ToString + Send + Sync + 'static> {
    value: T,
    // positive | mixed
    low_threshold: T,
    // mixed | high_threshold
    high_threshold: T,
}

#[derive(Component)]
struct IntelligenceSuspicionUi;

#[derive(Component)]
struct ScientificSuspicionUi;

#[derive(Component)]
struct RegionSuspicionUi;

#[derive(Component)]
struct PoliceSuspicionUi;

#[derive(Component)]
struct MediaSuspicionUi;

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
    game_date: Res<GameDate>,
    cult_name: Res<CultName>,
    cult_symbol: Res<CultSymbol>,
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
            BorderColor::all(BORDER_HIGHLIGHT),
            BackgroundColor::from(MENU_BACKGROUND),
            Visibility::Hidden,
            ZIndex(1),
        ))
        .id();

    let text_font = TextFont::from_font_size(SUB_HEADING).with_font(font_handle.0.clone());
    let unicode_text_font =
        TextFont::from_font_size(SUB_HEADING).with_font(unicode_font_handle.0.clone());

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
                    // Cult symbol
                    parent.spawn((
                        Node {
                            margin: UiRect::right(px(5)),
                            ..default()
                        },
                        Text::new(cult_symbol.0),
                        TextColor::from(TEXT),
                        unicode_text_font.clone(),
                    ));
                    // Funds counter
                    parent.spawn((
                        Node {
                            min_width: px(75),
                            ..default()
                        },
                        text_font.clone(),
                        // will be updated by funds_changed
                        TextKey::new("funds-display").add_arg("funds", 0),
                        TextColor::from(TEXT),
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
                            min_width: px(125),
                            ..default()
                        },
                        text_font.clone(),
                        TextColor::from(TEXT),
                        // will be updated by update_game_date
                        TextKey::new("game-date-display").add_arg("date", game_date.0),
                        GameDateUi,
                    ));
                    // Suspicion meters
                    parent.spawn((
                        Node {
                            min_width: px(50),
                            ..default()
                        },
                        text_font.clone(),
                        TextLayout::new_with_justify(Justify::Right),
                        MeterDisplay::<u32> {
                            value: 0,
                            low_threshold: 34,
                            high_threshold: 67,
                        },
                        IntelligenceSuspicionUi,
                    ));
                    parent.spawn((
                        Node {
                            min_width: px(50),
                            ..default()
                        },
                        text_font.clone(),
                        TextLayout::new_with_justify(Justify::Right),
                        MeterDisplay::<u32> {
                            value: 0,
                            low_threshold: 34,
                            high_threshold: 67,
                        },
                        ScientificSuspicionUi,
                    ));
                    // Separate left-aligned and right-aligned status fields
                    parent.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });
                    parent.spawn((
                        Node {
                            margin: UiRect::right(px(10)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        Text::new(&cult_name.0),
                        TextColor::from(TEXT),
                        TextFont::from_font_size(SMALL).with_font(font_handle.0.clone()),
                    ));
                    parent
                        .spawn((
                            Button,
                            GameSpeedAction::TogglePause,
                            Node {
                                width: px(25),
                                ..default()
                            },
                            // 23F8 DOUBLE VERTICAL BAR would be better but is not in the font.
                            // DOUBLE VERTICAL LINE
                            Text("\u{2016}".to_string()),
                            TextColor::from(TEXT),
                            unicode_text_font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .observe(on_game_speed_clicked);
                    parent
                        .spawn((
                            Button,
                            GameSpeedAction::SetSpeed(GameSpeed::Normal),
                            Node {
                                width: px(25),
                                ..default()
                            },
                            // RIGHTWARDS ARROW
                            Text("\u{2192}".to_string()),
                            TextColor::from(TEXT_HIGHLIGHT),
                            unicode_text_font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .observe(on_game_speed_clicked);
                    parent
                        .spawn((
                            Button,
                            GameSpeedAction::SetSpeed(GameSpeed::Fast),
                            Node {
                                width: px(25),
                                ..default()
                            },
                            // RIGHTWARDS PAIRED ARROWS
                            Text("\u{21C9}".to_string()),
                            TextColor::from(TEXT),
                            unicode_text_font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .observe(on_game_speed_clicked);
                    parent
                        .spawn((
                            Button,
                            GameSpeedAction::SetSpeed(GameSpeed::Faster),
                            Node {
                                width: px(25),
                                ..default()
                            },
                            // THREE RIGHTWARDS ARROWS
                            Text("\u{21F6}".to_string()),
                            TextColor::from(TEXT),
                            unicode_text_font.clone(),
                            TextLayout::new_with_justify(Justify::Center),
                        ))
                        .observe(on_game_speed_clicked);
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
}

fn update_game_date(
    game_date: Res<GameDate>,
    mut text_key: Single<&mut TextKey, With<GameDateUi>>,
) {
    text_key.replace_arg("date", game_date.0);
}

fn update_funds(funds: Res<Funds>, mut text_key: Single<&mut TextKey, With<FundsUi>>) {
    text_key.replace_arg("funds", funds.0);
}

fn setup_regions(
    mut commands: Commands,
    map_ui: Single<Entity, With<MapUi>>,
    regions: Query<(Entity, &Region, &Location, &Children)>,
    base_plots: Query<&Location, With<BasePlot>>,
    display_font_handle: Res<DisplayFontHandle>,
    font_handle: Res<FontHandle>,
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
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(px(1)),
                    border_radius: BorderRadius::all(px(10)),
                    padding: UiRect::all(px(5)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                UiTransform {
                    translation: Val2::percent(-50.0, -50.0),
                    ..default()
                },
                RegionUi,
                BorderColor::all(BORDER),
                BackgroundColor::from(MENU_BACKGROUND.with_alpha(0.75)),
            ))
            .observe(on_label_over)
            .observe(on_label_out)
            .with_children(|parent| {
                parent.spawn((
                    region.get_text_key(),
                    TextFont::from_font_size(SUB_HEADING).with_font(display_font_handle.0.clone()),
                ));
                parent.spawn((
                    ViewOf(entity),
                    RegionSuspicionUi,
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        column_gap: px(10),
                        display: Display::None,
                        ..default()
                    },
                    children![
                        (
                            TextFont::from_font_size(SMALL).with_font(font_handle.0.clone()),
                            MeterDisplay::<u32> {
                                value: 0,
                                low_threshold: 34,
                                high_threshold: 67,
                            },
                            PoliceSuspicionUi,
                            ViewOf(entity),
                        ),
                        (
                            TextFont::from_font_size(SMALL).with_font(font_handle.0.clone()),
                            MeterDisplay::<u32> {
                                value: 0,
                                low_threshold: 34,
                                high_threshold: 67,
                            },
                            MediaSuspicionUi,
                            ViewOf(entity),
                        )
                    ],
                ));
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
                    ..default()
                },
                UiTransform {
                    translation: Val2::percent(-50.0, -50.0),
                    ..default()
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
    bases: Query<&ChildOf, With<Base>>,
    base_plots: Query<(&ChildOf, &Views), With<BasePlot>>,
    regions: Query<&Views, With<Region>>,
    mut region_suspicion_uis: Query<&mut Node, With<RegionSuspicionUi>>,
    base_plot_uis: Query<&BasePlotUi>,
    base_types: Query<&Basetype, With<Base>>,
    font_handle: Res<FontHandle>,
) {
    let base_plot = bases.get(event.entity).unwrap().0;
    let (region, base_plot_views) = base_plots.get(base_plot).unwrap();
    let region_views = regions.get(region.0).unwrap();
    let mut region_suspicion_ui_node = region_views
        .iter()
        .find(|view| region_suspicion_uis.contains(*view))
        .map(|view| region_suspicion_uis.get_mut(view).unwrap())
        .unwrap();

    if region_suspicion_ui_node.display == Display::None {
        region_suspicion_ui_node.display = Display::Flex;
    }

    let base_plot_ui = base_plot_views
        .iter()
        .find(|view| base_plot_uis.contains(*view))
        .unwrap();
    let base_type = base_types.get(event.entity).unwrap();

    commands
        .spawn((
            ChildOf(base_plot_ui),
            ViewOf(event.entity),
            BaseUi,
            Node {
                flex_direction: FlexDirection::Column,
                border: UiRect::all(px(1)),
                border_radius: BorderRadius::all(px(5)),
                padding: UiRect::horizontal(px(2)),
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(WHITE),
            BackgroundColor::from(MENU_BACKGROUND.with_alpha(0.75)),
        ))
        .observe(on_label_over)
        .observe(on_label_out)
        .with_children(|parent| {
            parent.spawn((
                TextKey::new(format!("basetype-{}", &base_type.name)),
                TextFont::from_font_size(NORMAL).with_font(font_handle.0.clone()),
            ));
            parent.spawn((
                FollowerList,
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
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

    let text_font = TextFont::from_font_size(SMALL).with_font(unicode_font_handle.0.clone());

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

fn update_suspicion(
    intel_suspicion: Res<IntelligenceSuspicion>,
    scien_suspicion: Res<ScientificSuspicion>,
    mut intel_suspicion_ui: Single<
        &mut MeterDisplay<u32>,
        (
            With<IntelligenceSuspicionUi>,
            Without<ScientificSuspicionUi>,
        ),
    >,
    mut scien_suspicion_ui: Single<
        &mut MeterDisplay<u32>,
        (
            With<ScientificSuspicionUi>,
            Without<IntelligenceSuspicionUi>,
        ),
    >,
) {
    intel_suspicion_ui.value = intel_suspicion.0;
    scien_suspicion_ui.value = scien_suspicion.0;
}

fn update_regional_suspicion(
    regions: Query<
        (&Views, &PoliceSuspicion, &MediaSuspicion),
        (
            With<Region>,
            Or<(Changed<PoliceSuspicion>, Changed<MediaSuspicion>)>,
        ),
    >,
    mut police_suspicion_uis: Query<
        &mut MeterDisplay<u32>,
        (With<PoliceSuspicionUi>, Without<MediaSuspicionUi>),
    >,
    mut media_suspicion_uis: Query<
        &mut MeterDisplay<u32>,
        (With<MediaSuspicionUi>, Without<PoliceSuspicionUi>),
    >,
) {
    for (views, police, media) in regions.iter() {
        for view in views.0.iter() {
            if let Ok(mut police_suspicion_meter) = police_suspicion_uis.get_mut(*view) {
                police_suspicion_meter.value = police.0;
            }
            if let Ok(mut media_suspicion_meter) = media_suspicion_uis.get_mut(*view) {
                media_suspicion_meter.value = media.0;
            }
        }
    }
}

fn update_meter_display<T: PartialOrd + ToString + Send + Sync + 'static>(
    mut meters: Query<(&mut Text, &mut TextColor, &MeterDisplay<T>), Changed<MeterDisplay<T>>>,
) {
    for (mut text, mut text_color, meter) in meters.iter_mut() {
        text.0 = meter.value.to_string();

        if meter.low_threshold < meter.high_threshold {
            // POS | MIX | NEG
            *text_color = if meter.value < meter.low_threshold {
                TEXT_POSITIVE
            } else if meter.value >= meter.low_threshold && meter.value < meter.high_threshold {
                TEXT_MIXED
            } else {
                TEXT_NEGATIVE
            }
            .into();
        } else {
            // NEG | MIX | POS
            *text_color = if meter.value < meter.high_threshold {
                TEXT_POSITIVE
            } else if meter.value >= meter.high_threshold && meter.value < meter.low_threshold {
                TEXT_MIXED
            } else {
                TEXT_NEGATIVE
            }
            .into();
        }
    }
}

fn on_game_speed_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    game_speed_actions: Query<&GameSpeedAction>,
) {
    if click.button == PointerButton::Primary {
        let game_speed_action = *game_speed_actions.get(click.entity).unwrap();
        commands.trigger(GameSpeedChangedEvent(game_speed_action));
    }
}

fn on_label_over(
    event: On<Pointer<Over>>,
    mut label_colors: Query<(&mut BackgroundColor, &mut BorderColor)>,
) {
    let (mut background_color, mut border_color) = label_colors.get_mut(event.entity).unwrap();
    border_color.set_all(BORDER_HIGHLIGHT);
    background_color.0.set_alpha(1.0);
}

fn on_label_out(
    event: On<Pointer<Out>>,
    mut label_colors: Query<(&mut BackgroundColor, &mut BorderColor)>,
) {
    let (mut background_color, mut border_color) = label_colors.get_mut(event.entity).unwrap();
    border_color.set_all(BORDER);
    background_color.0.set_alpha(0.75);
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

fn update_funds_tooltip(
    mut commands: Commands,
    incomes: Query<&Income>,
    expenses: Query<&Expense>,
    tooltip: Single<Entity, With<FundsTooltip>>,
    font_handle: Res<FontHandle>,
) {
    fn income_expense_row(
        mut commands: Commands,
        parent: Entity,
        text_font: &TextFont,
        category: String,
        count: usize,
        funds: FundsAmount,
    ) {
        commands
            .spawn((
                // Node to represent the row
                Node::default(),
                ChildOf(parent),
            ))
            .with_children(|parent| {
                parent.spawn((Text::new(format!("{count}x ")), text_font.clone()));
                parent.spawn((TextKey::new(category), text_font.clone()));
                parent.spawn(Node {
                    flex_grow: 1.0,
                    padding: UiRect::left(px(5)),
                    ..default()
                });
                parent.spawn((
                    TextKey::new("funds").add_arg("funds", funds),
                    text_font.clone(),
                ));
            });
    }

    let tooltip = tooltip.entity();
    commands.entity(tooltip).despawn_children();

    // Completely refresh the tooltip contents
    let text_font = TextFont::from_font_size(NORMAL).with_font(font_handle.0.clone());
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
        TextKey::new("income-tooltip-header"),
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
            );
        }
    }
    commands.spawn((
        TextKey::new("expense-tooltip-header"),
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
            );
        }
    }
}
