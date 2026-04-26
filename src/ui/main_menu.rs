use bevy::prelude::*;

use crate::{
    constants::ui::*,
    main_menu::NewGame,
    state::GameState,
    text::TextKey,
    ui::{DisplayFontHandle, FontHandle, save_load::open_load_game_popup},
};

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    font_handle: Res<FontHandle>,
    display_font_handle: Res<DisplayFontHandle>,
) {
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
            BorderColor::all(WHITE),
            BackgroundColor::from(MENU_BACKGROUND),
            children![(
                TextFont::from_font_size(60.0).with_font(font_handle.0.clone()),
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
                    TextFont::from_font_size(150.0).with_font(display_font_handle.0.clone()),
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
                    parent.spawn(button("menu-button-load-game")).observe(
                        |click: On<Pointer<Click>>,
                         mut commands: Commands,
                         font: Res<FontHandle>| {
                            if click.button == PointerButton::Primary {
                                open_load_game_popup(commands.reborrow(), font.0.clone());
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
