use bevy::prelude::*;

use crate::ui::{FontHandle, dialog::DialogBuilder};

pub fn warn_no_save(mut commands: Commands, font: Res<FontHandle>) {
    DialogBuilder::new(font.0.clone())
        .with_pause()
        .with_title("save-error-title")
        .with_text_body("save-error-body")
        .with_confirm_label("dialog-ok")
        .build(&mut commands);
}
