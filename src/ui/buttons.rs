use bevy::{input_focus::InputFocus, prelude::*};

use crate::constants::ui::{MENU_BACKGROUND, MENU_HOVER_BACKGROUND, MENU_PRESSED_BACKGROUND};

pub fn setup_observe_buttons(mut commands: Commands) {
    commands.add_observer(
        |over: On<Pointer<Over>>,
         mut buttons: Query<&mut BackgroundColor, With<Button>>,
         mut input_focus: ResMut<InputFocus>| {
            if let Ok(mut background) = buttons.get_mut(over.entity) {
                background.0 = MENU_HOVER_BACKGROUND.into();
            }
            input_focus.set(over.entity);
        },
    );
    commands.add_observer(
        |out: On<Pointer<Out>>,
         mut buttons: Query<&mut BackgroundColor, With<Button>>,
         mut input_focus: ResMut<InputFocus>| {
            if let Ok(mut background) = buttons.get_mut(out.entity) {
                background.0 = MENU_BACKGROUND.into();
            }
            input_focus.clear();
        },
    );
    commands.add_observer(
        |press: On<Pointer<Press>>, mut buttons: Query<&mut BackgroundColor, With<Button>>| {
            if let Ok(mut background) = buttons.get_mut(press.entity) {
                background.0 = MENU_PRESSED_BACKGROUND.into();
            }
        },
    );
    commands.add_observer(
        |click: On<Pointer<Click>>, mut buttons: Query<(&mut BackgroundColor, &mut Button)>| {
            if let Ok((mut background, mut button)) = buttons.get_mut(click.entity) {
                background.0 = MENU_HOVER_BACKGROUND.into();
                button.set_changed();
            }
        },
    );
}
