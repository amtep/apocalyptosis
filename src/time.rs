use bevy::prelude::*;
use chrono::{Days, NaiveDate};

use crate::state::{GameState, MainSetupSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        setup.in_set(MainSetupSet::Default),
    )
    .init_resource::<CurrentGameSpeed>()
    .add_systems(FixedUpdate, fixed_update.run_if(in_state(GameState::Main)))
    .add_systems(Update, listen_speed_keys.run_if(in_state(GameState::Main)));
}

#[derive(Resource)]
pub struct GameDate(pub NaiveDate);

impl Default for GameDate {
    fn default() -> Self {
        Self(NaiveDate::from_ymd_opt(2026, 4, 15).unwrap())
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Time::<Fixed>::from_seconds(1.0));
    commands.insert_resource(GameDate::default());
    commands.add_observer(on_game_speed_changed);
}

#[derive(Resource, Default)]
pub struct CurrentGameSpeed {
    pub paused: bool,
    pub speed: GameSpeed,
}

#[derive(Event)]
pub struct GameDateChangedEvent;

fn fixed_update(mut commands: Commands, mut date: ResMut<GameDate>) {
    // We don't expect to reach 262000 AD
    date.0 = date.0 + Days::new(1);
    commands.trigger(GameDateChangedEvent);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GameSpeed {
    #[default]
    Normal,
    Fast,
    Faster,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
pub enum GameSpeedAction {
    SetSpeed(GameSpeed),
    TogglePause,
}

#[derive(Event)]
pub struct GameSpeedChangedEvent(pub GameSpeedAction);

fn on_game_speed_changed(
    event: On<GameSpeedChangedEvent>,
    mut time: ResMut<Time<Virtual>>,
    mut current_game_speed: ResMut<CurrentGameSpeed>,
) {
    match event.0 {
        GameSpeedAction::SetSpeed(speed) => {
            let s = match speed {
                GameSpeed::Normal => 1.0,
                GameSpeed::Fast => 2.0,
                GameSpeed::Faster => 5.0,
            };
            info!("Game speed to {s}");
            time.set_relative_speed(s);
            time.unpause();
            *current_game_speed = CurrentGameSpeed {
                paused: false,
                speed,
            };
        }
        GameSpeedAction::TogglePause => {
            if current_game_speed.paused {
                info!("Unpausing");
                time.unpause();
            } else {
                info!("Pausing");
                time.pause();
            }
            current_game_speed.paused = !current_game_speed.paused;
        }
    }
}

fn listen_speed_keys(mut commands: Commands, keys: Res<ButtonInput<KeyCode>>) {
    let action = if keys.just_pressed(KeyCode::Digit1) {
        GameSpeedAction::SetSpeed(GameSpeed::Normal)
    } else if keys.just_pressed(KeyCode::Digit2) {
        GameSpeedAction::SetSpeed(GameSpeed::Fast)
    } else if keys.just_pressed(KeyCode::Digit3) {
        GameSpeedAction::SetSpeed(GameSpeed::Faster)
    } else if keys.just_pressed(KeyCode::Space) {
        GameSpeedAction::TogglePause
    } else {
        return;
    };

    commands.trigger(GameSpeedChangedEvent(action));
}
