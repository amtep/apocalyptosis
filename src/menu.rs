use bevy::prelude::*;

use crate::state::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup)
            .add_systems(OnExit(GameState::Menu), cleanup)
            .add_systems(Update, interaction.run_if(in_state(GameState::Menu)));
    }
}

fn setup() {}

fn cleanup() {}

fn interaction() {}
