use bevy::{input_focus::InputFocus, prelude::*};
use chrono::{Days, NaiveDate};

use crate::state::{GameState, MainSetupSet};

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Main),
            setup.in_set(MainSetupSet::Default),
        )
        .add_systems(FixedUpdate, fixed_update.run_if(in_state(GameState::Main)))
        .add_systems(
            Update,
            (update_speed_buttons, listen_speed_keys).run_if(in_state(GameState::Main)),
        );
    }
}

#[derive(Resource)]
pub struct GameDate(pub NaiveDate);

impl Default for GameDate {
    fn default() -> Self {
        Self(NaiveDate::from_ymd_opt(2026, 4, 15).unwrap())
    }
}

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Time::<Fixed>::from_seconds(1.0));
    commands.insert_resource(GameDate::default());
}

#[derive(Event)]
pub struct GameDateChangedEvent;

pub fn fixed_update(mut commands: Commands, mut date: ResMut<GameDate>) {
    // We don't expect to reach 262000 AD
    date.0 = date.0 + Days::new(1);
    commands.trigger(GameDateChangedEvent);
}

#[derive(Component)]
pub struct GameSpeed(pub f64);

pub fn update_speed_buttons(
    mut input_focus: ResMut<InputFocus>,
    mut speed: ResMut<Time<Fixed>>,
    mut q: Query<(Entity, &Interaction, &mut Button, &GameSpeed), Changed<Interaction>>,
) {
    for (entity, interaction, mut button, game_speed) in &mut q {
        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                // alert the accessibility system
                button.set_changed();
                info!("Game speed {}", game_speed.0);
                speed.set_timestep_seconds(1.0 / game_speed.0);
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
            }
        }
    }
}

// TODO: sync with the buttons somehow, to avoid duplicating the speed settings.
pub fn listen_speed_keys(keys: Res<ButtonInput<KeyCode>>, mut speed: ResMut<Time<Fixed>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        info!("Game speed 1");
        speed.set_timestep_seconds(1.0);
    } else if keys.just_pressed(KeyCode::Digit2) {
        info!("Game speed 2");
        speed.set_timestep_seconds(0.5);
    } else if keys.just_pressed(KeyCode::Digit3) {
        info!("Game speed 5");
        speed.set_timestep_seconds(0.2);
    }
}
