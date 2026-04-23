use bevy::prelude::*;

use crate::constants::ui::{MENU_BACKGROUND, MENU_HOVER_BACKGROUND, MENU_PRESSED_BACKGROUND};

pub fn update_button_backgrounds(
    mut q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut background_color) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                *background_color = MENU_PRESSED_BACKGROUND.into();
            }
            Interaction::Hovered => {
                *background_color = MENU_HOVER_BACKGROUND.into();
            }
            Interaction::None => {
                *background_color = MENU_BACKGROUND.into();
            }
        }
    }
}
