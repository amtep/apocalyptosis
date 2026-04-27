use bevy::prelude::*;

use crate::state::{GameState, MainSetupSet};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        remove_new_game
            .run_if(resource_exists::<NewGame>)
            .in_set(MainSetupSet::Late),
    );
}

#[derive(Resource, Default)]
pub struct NewGame;

fn remove_new_game(mut commands: Commands) {
    commands.remove_resource::<NewGame>();
}
