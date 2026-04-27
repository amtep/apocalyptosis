use bevy::prelude::*;

use crate::{
    constants::ui::{BORDER, NORMAL},
    save_load::scan_saved_games,
    text::TextKey,
    ui::dialog::DialogBuilder,
};

pub fn warn_no_save(mut commands: Commands, font: Handle<Font>) {
    DialogBuilder::new(font)
        .with_pause()
        .with_title("save-error-title")
        .with_text_body("save-error-body")
        .with_confirm_label("dialog-ok")
        .build(&mut commands);
}

fn warn_no_load_scan(mut commands: Commands, font: Handle<Font>) {
    DialogBuilder::new(font)
        .with_title("load-scan-error-title")
        .with_text_body("load-scan-error-body")
        .with_confirm_label("dialog-ok")
        .with_cancel_label("dialog-back")
        .build(&mut commands);
}

fn warn_no_load(mut commands: Commands, font: Handle<Font>) {
    DialogBuilder::new(font)
        .with_title("load-error-title")
        .with_text_body("load-error-body")
        .with_confirm_label("dialog-ok")
        .with_cancel_label("dialog-back")
        .build(&mut commands);
}

pub fn open_load_game_popup(mut commands: Commands, font: Handle<Font>) {
    let mut v = match scan_saved_games() {
        Err(e) => {
            error!("Could not scan saved games: {e}");
            warn_no_load_scan(commands.reborrow(), font);
            return;
        }
        Ok(v) => v,
    };
    v.sort_by_key(|(_, metadata, _)| metadata.save_timestamp);
    v.reverse();
    let body = commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            width: percent(80.0),
            row_gap: px(4),
            ..default()
        })
        .id();
    let text_font = TextFont::from_font_size(NORMAL).with_font(font.clone());
    for (campaign, metadata, content) in v {
        commands
            .spawn((
                Node {
                    border: UiRect::all(px(2)),
                    ..default()
                },
                BorderColor::all(BORDER),
                ChildOf(body),
            ))
            .with_child((Text(format!("{}", *campaign)), text_font.clone()))
            .with_child(Node {
                flex_grow: 1.0,
                ..default()
            })
            .with_child((
                TextKey::new("saved-game-date").add_arg("date", metadata.save_timestamp),
                text_font.clone(),
            ));
    }
    DialogBuilder::new(font)
        .with_title("load-game-title")
        .with_entity_body(body)
        .with_cancel_label("dialog-back")
        .build(&mut commands);
}
