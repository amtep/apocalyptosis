use bevy::prelude::*;

use crate::{
    constants::ui::{FONT_DISPLAY_PATH, FONT_PATH, MENU_BACKGROUND, TEXTURE_EARTH_BACKGROUND},
    state::GameState,
    text::{FluentBundleWrapper, TextKey},
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(
            Update,
            listen_button_press.run_if(in_state(GameState::MainMenu)),
        );
}

#[derive(Component)]
enum MenuAction {
    NewGame,
    Quit,
}

fn setup(mut commands: Commands, bundle: Res<FluentBundleWrapper>, asset_server: Res<AssetServer>) {
    let button = |key, action| {
        (
            Button,
            Node {
                width: percent(100),
                padding: UiRect::all(px(20)),
                border: UiRect::all(px(4)),
                border_radius: BorderRadius::all(px(40)),
                align_self: AlignSelf::Center,
                justify_self: JustifySelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(MENU_BACKGROUND.into()),
            action,
            children![(
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    font_size: 60.0,
                    ..default()
                },
                TextKey::new(key, &bundle),
            )],
        )
    };
    commands.spawn((
        DespawnOnExit(GameState::MainMenu),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Column,
            row_gap: px(40),
            ..default()
        },
        ImageNode {
            image: asset_server.load(TEXTURE_EARTH_BACKGROUND),
            image_mode: NodeImageMode::Stretch,
            ..default()
        },
        children![
            (
                Node {
                    width: percent(100),
                    height: px(200),
                    padding: UiRect::all(px(10)),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    TextKey::new("main-menu-title", &bundle),
                    TextFont {
                        font: asset_server.load(FONT_DISPLAY_PATH),
                        font_size: 140.0,
                        ..default()
                    },
                    TextShadow::default(),
                )],
            ),
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    row_gap: px(40),
                    ..default()
                },
                children![
                    button("menu-button-new-game", MenuAction::NewGame),
                    button("menu-button-quit", MenuAction::Quit),
                ]
            ),
        ],
    ));
}

fn listen_button_press(
    q: Query<(&Interaction, &MenuAction), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, menu_action) in &q {
        if *interaction == Interaction::Pressed {
            match menu_action {
                MenuAction::NewGame => {
                    game_state.set(GameState::Main);
                }
                MenuAction::Quit => {
                    exit.write(AppExit::Success);
                }
            }
        }
    }
}
