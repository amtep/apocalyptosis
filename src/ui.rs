use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use chrono::Datelike;
use fluent::FluentArgs;
use fluent_datetime::{FluentDateTime, length};
use icu::{
    calendar::Date,
    time::{DateTime, Time},
};

use crate::{
    text::FluentBundleResource,
    time::{GameDate, GameDateChanged},
};

#[derive(Component)]
pub struct MapUi;

#[derive(Component)]
struct GameDateUi;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: vw(100.0),
            height: vh(100.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        width: vw(100.0),
                        height: vh(5.0),
                        border: UiRect {
                            left: vw(0.0),
                            right: vw(0.0),
                            top: vh(0.5),
                            bottom: vh(0.5),
                        },
                        ..default()
                    },
                    BorderColor::all(WHITE),
                    BackgroundColor(BLACK.into()),
                ))
                .with_children(|parent| {
                    // TODO: figure out how to get the date display on the right of the screen.
                    parent.spawn((
                        // will be updated by update_game_date_display()
                        Text(String::new()),
                        TextLayout::new_with_justify(Justify::Right),
                        Node {
                            align_self: AlignSelf::FlexEnd,
                            ..default()
                        },
                        GameDateUi,
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
    commands.add_observer(update_game_date_display);
}

fn update_game_date_display(
    _: On<GameDateChanged>,
    date: Res<GameDate>,
    mut text: Single<&mut Text, With<GameDateUi>>,
    fluent: Res<FluentBundleResource>,
) {
    let key = "game-date-display";

    // Do a little dance
    let date = Date::try_new_iso(date.0.year(), date.0.month() as u8, date.0.day() as u8).unwrap();
    let datetime = DateTime {
        date: date,
        time: Time::start_of_day(),
    };
    let mut datetime: FluentDateTime = datetime.into();
    datetime.options.set_date_style(Some(length::Date::Long));

    if let Some(pattern) = fluent.get_pattern(key, &text.0) {
        let mut errors = Vec::new();
        let mut args = FluentArgs::new();
        args.set("date", datetime);
        let new_text = fluent.0.format_pattern(pattern, Some(&args), &mut errors);
        for err in errors {
            warn!("error evaluating key: {key}: {err}");
        }
        text.0 = new_text.into_owned();
    }
}
