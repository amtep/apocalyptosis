use bevy::{color::palettes::css::WHITE, prelude::*};
use chrono::Datelike;
use fluent::FluentArgs;
use fluent_datetime::{FluentDateTime, length};
use icu::{
    calendar::Date,
    time::{DateTime, Time},
};

use crate::{
    constants::ui::{
        FONT_PATH, MENU_BACKGROUND, MENU_HOVER_BACKGROUND, MENU_PRESSED_BACKGROUND,
        UNICODE_FONT_PATH,
    },
    funds::{Funds, FundsAmount, FundsChanged},
    text::FluentBundleResource,
    time::{GameDate, GameDateChanged, GameSpeed},
};

#[derive(Component)]
pub struct MapUi;

#[derive(Component)]
struct GameDateUi;

#[derive(Component)]
struct FundsUi;

#[derive(Component)]
#[require(Text)]
pub struct FundsDisplay(pub FundsAmount);

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load(FONT_PATH);
    let unicode_font = asset_server.load(UNICODE_FONT_PATH);
    commands.spawn(Camera2d);
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
                        ..default()
                    },
                    BorderColor::all(WHITE),
                    BackgroundColor(MENU_BACKGROUND.into()),
                ))
                .with_children(|parent| {
                    // Separate left-aligned and right-aligned status fields
                    parent.spawn(Node {
                        flex_grow: 1.0,
                        ..default()
                    });
                    // Funds counter
                    parent.spawn((
                        Node {
                            padding: UiRect::right(px(5)),
                            ..default()
                        },
                        // will be updated by update_funds_display()
                        Text(String::new()),
                        TextFont {
                            font: font.clone(),
                            ..default()
                        },
                        FundsUi,
                    ));
                    // Game date display
                    parent.spawn((
                        Node {
                            padding: UiRect::right(px(5)),
                            ..default()
                        },
                        // will be updated by update_game_date_display()
                        Text(String::new()),
                        TextFont {
                            font: font.clone(),
                            ..default()
                        },
                        GameDateUi,
                    ));
                    parent.spawn((
                        Button,
                        GameSpeed(1.0),
                        // RIGHTWARDS ARROW
                        Text("\u{2192}".to_string()),
                        TextFont {
                            font: unicode_font.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Button,
                        GameSpeed(2.0),
                        // RIGHTWARDS PAIRED ARROWS
                        Text("\u{21C9}".to_string()),
                        TextFont {
                            font: unicode_font.clone(),
                            ..default()
                        },
                    ));
                    parent.spawn((
                        Button,
                        GameSpeed(5.0),
                        // THREE RIGHTWARDS ARROWS
                        Text("\u{21F6}".to_string()),
                        TextFont {
                            font: unicode_font.clone(),
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
    commands.add_observer(update_game_date_display);
    commands.add_observer(update_funds_display);
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
        date,
        time: Time::start_of_day(),
    };
    let mut datetime: FluentDateTime = datetime.into();
    datetime.options.set_date_style(Some(length::Date::Long));

    if let Some(pattern) = fluent.get_pattern(key, &text.0) {
        let mut args = FluentArgs::new();
        args.set("date", datetime);
        fluent.format_pattern(key, pattern, Some(&args), &mut text.0);
    }
}

fn update_funds_display(
    _: On<FundsChanged>,
    funds: Res<Funds>,
    mut text: Single<&mut Text, With<FundsUi>>,
    fluent: Res<FluentBundleResource>,
) {
    let key = "funds-display";
    if let Some(pattern) = fluent.get_pattern(key, &text.0) {
        let mut args = FluentArgs::new();
        args.set("funds", funds.0);
        fluent.format_pattern(key, pattern, Some(&args), &mut text.0);
    }
}

pub fn update_funds_displays(
    mut q: Query<(&mut Text, &FundsDisplay), Added<FundsDisplay>>,
    fluent: Res<FluentBundleResource>,
) {
    let key = "funds";
    for (mut text, funds) in &mut q {
        if let Some(pattern) = fluent.get_pattern(key, &text.0) {
            let mut args = FluentArgs::new();
            args.set("funds", funds.0);
            fluent.format_pattern(key, pattern, Some(&args), &mut text.0);
        }
    }
}

pub fn update_button_colors(
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
