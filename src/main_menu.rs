use bevy::prelude::*;

use crate::state::{GameState, MainSetupSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        remove_new_or_loaded_game.in_set(MainSetupSet::Late),
    );
}

#[derive(Resource, Default)]
pub struct NewGame;

#[derive(Resource, Default)]
pub struct LoadedGame;

fn remove_new_or_loaded_game(mut commands: Commands) {
    commands.remove_resource::<NewGame>();
    commands.remove_resource::<LoadedGame>();
}
