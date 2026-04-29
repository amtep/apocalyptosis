use bevy::prelude::*;

use crate::{
    constants::ui::*,
    regions::{BasePlot, Location, Region},
    suspicion::{MediaSuspicion, PoliceSuspicion},
};

use super::{
    DisplayFontHandle, FontHandle, MapUi, MeterDisplay, ViewOf, Views, on_changed_follower,
    on_label_out, on_label_over, on_spawn_base,
};

#[derive(Component)]
pub struct RegionSuspicionUi;

#[derive(Component)]
pub struct PoliceSuspicionUi;

#[derive(Component)]
pub struct MediaSuspicionUi;

#[derive(Component)]
struct RegionUi;

#[derive(Component)]
pub struct BasePlotUi;

pub fn setup(
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

pub fn update_regional_suspicion(
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
