use bevy::{
    input::keyboard::KeyboardInput,
    input_focus::{FocusedInput, InputFocus},
    prelude::*,
    ui::InteractionDisabled,
};

use crate::{
    constants::ui::colors::*,
    state::GameState,
    ui::{MapUi, menu::Menu},
};

pub fn plugin(app: &mut App) {
    app.add_systems(Update, button_focus)
        .add_systems(OnExit(GameState::Load), setup_observe_buttons);
}

/// Sent when a [`Button`] is activated, either by pointer click or by keypress.
#[derive(EntityEvent)]
pub struct Clicked(Entity);

fn setup_observe_buttons(mut commands: Commands) {
    commands.add_observer(
        |over: On<Pointer<Over>>,
         mut buttons: Query<
            (
                &mut BackgroundColor,
                Has<InteractionDisabled>,
            ),
            With<Button>,
        >| {
            if let Ok((mut background, has_interaction_disabled)) =
                buttons.get_mut(over.entity)
                && !has_interaction_disabled
            {
                background.0 = BUTTON_HOVER_BACKGROUND
                    .with_alpha(background.0.alpha())
                    .into();
            }
        },
    );
    commands.add_observer(
        |out: On<Pointer<Out>>,
         mut buttons: Query<
            (
                &mut BackgroundColor,
                Has<InteractionDisabled>,
            ),
            With<Button>,
        >| {
            if let Ok((mut background, has_interaction_disabled)) =
                buttons.get_mut(out.entity)
                && !has_interaction_disabled
            {
                background.0 = BUTTON_BACKGROUND.with_alpha(background.0.alpha()).into();
            }
        },
    );
    commands.add_observer(
        |press: On<Pointer<Press>>, mut buttons: Query<(&mut BackgroundColor, Has<InteractionDisabled>), With<Button>>| {
            if press.button == PointerButton::Primary
                && let Ok((mut background, has_interaction_disabled)) = buttons.get_mut(press.entity)
                && !has_interaction_disabled
            {
                background.0 = BUTTON_PRESSED_BACKGROUND.with_alpha(background.0.alpha()).into();
            }
        },
    );
    commands.add_observer(
        |click: On<Pointer<Click>>, mut commands: Commands, mut buttons: Query<(&mut BackgroundColor, &mut Button, Has<InteractionDisabled>)>| {
            if click.button == PointerButton::Primary
                && let Ok((mut background, mut button, has_interaction_disabled)) = buttons.get_mut(click.entity)
                && !has_interaction_disabled
            {
                background.0 = BUTTON_HOVER_BACKGROUND.with_alpha(background.0.alpha()).into();
                button.set_changed();
                commands.entity(click.event_target()).trigger(Clicked);
            }
        },
    );
    commands.add_observer(
        |ev: On<FocusedInput<KeyboardInput>>,
         mut commands: Commands,
         mut buttons: Query<(&mut Button, Has<InteractionDisabled>)>| {
            if let Ok((mut button, has_interaction_disabled)) = buttons.get_mut(ev.event_target())
                && !has_interaction_disabled
                && ev.input.key_code == KeyCode::Enter
            {
                commands.entity(ev.event_target()).trigger(Clicked);
                button.set_changed();
            }
        },
    );
    commands.add_observer(
        |mut drag: On<Pointer<Drag>>, buttons: Query<(), With<Button>>| {
            if buttons.contains(drag.entity) {
                drag.propagate(false);
            }
        },
    );

    commands.add_observer(
        |add: On<Add, InteractionDisabled>,
         mut buttons: Query<(&Children, &mut BackgroundColor), With<Button>>,
         mut text_colors: Query<&mut TextColor>| {
            if let Ok((children, mut background)) = buttons.get_mut(add.entity) {
                background.0 = BUTTON_BACKGROUND.with_alpha(background.0.alpha()).into();
                for child in children {
                    if let Ok(mut text_color) = text_colors.get_mut(*child) {
                        text_color.0 = TEXT_DISABLED.into();
                    }
                }
            }
        },
    );

    commands.add_observer(
        |remove: On<Remove, InteractionDisabled>,
         mut buttons: Query<(&Children, &mut BackgroundColor), With<Button>>,
         mut text_colors: Query<&mut TextColor>| {
            if let Ok((children, mut background)) = buttons.get_mut(remove.entity) {
                background.0 = BUTTON_BACKGROUND.with_alpha(background.0.alpha()).into();
                for child in children {
                    if let Ok(mut text_color) = text_colors.get_mut(*child) {
                        text_color.0 = TEXT.into();
                    }
                }
            }
        },
    );

    // click outside any menu should close all opened menus
    commands.add_observer(
        |click: On<Pointer<Click>>,
         mut commands: Commands,
         map_ui: Query<&MapUi>,
         menus: Query<Entity, With<Menu>>| {
            if click.button == PointerButton::Primary && map_ui.contains(click.entity) {
                for menu in &menus {
                    commands.entity(menu).try_despawn();
                }
            }
        },
    );
}

fn button_focus(
    input_focus: Res<InputFocus>,
    mut buttons: Query<(Entity, &mut BorderColor), With<Button>>,
) {
    if input_focus.is_changed() {
        for (e, mut border) in &mut buttons {
            if input_focus.0 == Some(e) {
                border.set_all(BORDER_HIGHLIGHT);
            } else {
                border.set_all(BORDER);
            }
        }
    }
}
