use bevy::prelude::*;

use crate::{
    constants::ui::{FONT_DISPLAY_PATH, FONT_PATH, MENU_BACKGROUND, TEXTURE_EARTH_BACKGROUND},
    state::{GameState, MainSetupSet},
    text::TextKey,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(
            OnEnter(GameState::Main),
            remove_new_game
                .run_if(resource_exists::<NewGame>)
                .in_set(MainSetupSet::Late),
        );
}

#[derive(Resource)]
pub struct NewGame;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button = |key| {
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
            children![(
                TextFont {
                    font: asset_server.load(FONT_PATH),
                    font_size: 60.0,
                    ..default()
                },
                TextKey::new(key),
            )],
        )
    };
    commands
        .spawn((
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
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: percent(100),
                    height: px(200),
                    padding: UiRect::all(px(10)),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    TextKey::new("main-menu-title"),
                    TextFont {
                        font: asset_server.load(FONT_DISPLAY_PATH),
                        font_size: 140.0,
                        ..default()
                    },
                    TextShadow::default(),
                )],
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    align_self: AlignSelf::Center,
                    row_gap: px(40),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(button("menu-button-new-game")).observe(
                        |click: On<Pointer<Click>>,
                         mut commands: Commands,
                         mut game_state: ResMut<NextState<GameState>>| {
                            if click.button == PointerButton::Primary {
                                commands.insert_resource(NewGame);
                                game_state.set(GameState::Main);
                            }
                        },
                    );
                    parent.spawn(button("menu-button-quit")).observe(
                        |click: On<Pointer<Click>>, mut exit: MessageWriter<AppExit>| {
                            if click.button == PointerButton::Primary {
                                exit.write(AppExit::Success);
                            }
                        },
                    );
                });
        });
}

fn remove_new_game(mut commands: Commands) {
    commands.remove_resource::<NewGame>();
}
