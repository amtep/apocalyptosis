use bevy::prelude::*;

use crate::{
    constants::ui::{BORDER, NORMAL, TEXT},
    main_menu::LoadedGame,
    save_load::{Campaign, load, scan_saved_games},
    state::GameState,
    text::TextKey,
    ui::{
        Selected,
        dialog::{Dialog, DialogConfirmed},
    },
};

#[derive(Component)]
struct LoadGameOption(Campaign, Vec<u8>);

#[must_use]
pub fn warn_no_save() -> Dialog {
    Dialog::new()
        .with_pause()
        .with_title("save-error-title")
        .with_text_body("save-error-body")
        .with_confirm_label("dialog-ok")
}

#[must_use]
fn warn_no_load_scan() -> Dialog {
    Dialog::new()
        .with_title("load-scan-error-title")
        .with_text_body("load-scan-error-body")
        .with_confirm_label("dialog-ok")
        .with_cancel_label("dialog-back")
}

#[must_use]
fn warn_no_load() -> Dialog {
    Dialog::new()
        .with_title("load-error-title")
        .with_text_body("load-error-body")
        .with_confirm_label("dialog-ok")
        .with_cancel_label("dialog-back")
}

pub fn open_load_game_popup(
    mut commands: Commands,
    font: Handle<Font>,
    unicode_font: Handle<Font>,
) {
    let mut v = match scan_saved_games() {
        Err(e) => {
            error!("Could not scan saved games: {e}");
            commands.spawn(warn_no_load_scan());
            return;
        }
        Ok(v) => v,
    };
    v.sort_by_key(|(_, metadata, _)| std::cmp::Reverse(metadata.save_timestamp));
    let body = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: percent(80.0),
            row_gap: px(4),
            ..default()
        })
        .id();
    let text_font = TextFont::from_font_size(NORMAL).with_font(font.clone());
    let unicode_font = TextFont::from_font_size(NORMAL).with_font(unicode_font.clone());
    for (campaign, metadata, content) in v {
        commands
            .spawn((
                Button,
                Node {
                    flex_direction: FlexDirection::Column,
                    border: UiRect::all(px(2)),
                    border_radius: BorderRadius::all(px(10.0)),
                    padding: UiRect::all(px(4)),
                    ..default()
                },
                BorderColor::all(BORDER),
                ChildOf(body),
                LoadGameOption(campaign, content),
                children![
                    (
                        Node::default(),
                        children![
                            (
                                Text(format!("{} ", metadata.cult_symbol)),
                                unicode_font.clone(),
                                TextColor(TEXT.into()),
                            ),
                            (
                                Text(metadata.cult_name),
                                text_font.clone(),
                                TextColor(TEXT.into()),
                            ),
                        ]
                    ),
                    (
                        Node::default(),
                        children![
                            (
                                TextKey::new("game-date-display")
                                    .add_arg("date", metadata.game_date),
                                text_font.clone(),
                                TextColor(TEXT.into())
                            ),
                            Node {
                                flex_grow: 1.0,
                                ..default()
                            },
                            (
                                TextKey::new("funds").add_arg("funds", metadata.funds),
                                text_font.clone(),
                                TextColor(TEXT.into())
                            ),
                        ]
                    ),
                    (
                        Node {
                            flex_direction: FlexDirection::RowReverse,
                            ..default()
                        },
                        children![(
                            TextKey::new("saved-game-date")
                                .add_arg("date", metadata.save_timestamp),
                            text_font.clone(),
                            TextColor(TEXT.into()),
                        )]
                    ),
                ],
            ))
            .observe(
                |click: On<Pointer<Click>>,
                 mut commands: Commands,
                 mut q: Query<(Entity, &mut Node), With<LoadGameOption>>| {
                    if click.button == PointerButton::Primary {
                        for (e, mut node) in &mut q {
                            commands.entity(e).remove::<Selected>();
                            node.border = UiRect::all(px(2));
                        }
                        commands.entity(click.entity).insert(Selected);
                        q.get_mut(click.entity).unwrap().1.border = UiRect::all(px(4));
                    }
                },
            );
    }
    commands
        .spawn(
            Dialog::new()
                .with_title("load-game-title")
                .with_entity_body(body)
                .with_confirm_label("load-game-confirm")
                .with_cancel_label("dialog-back"),
        )
        .observe(
            |_: On<Add, DialogConfirmed>,
             mut commands: Commands,
             option: Single<&LoadGameOption, With<Selected>>,
             mut next_state: ResMut<NextState<GameState>>| {
                let LoadGameOption(campaign, content) = *option;
                // Set the next state early, so that it can be set back to MainMenu
                // if the load fails. It won't take effect till the next frame anyway.
                next_state.set(GameState::Main);
                info!("Loading game {}", **campaign);
                load(commands.reborrow(), *campaign, content.clone());
                commands.insert_resource(LoadedGame);
            },
        );
}
