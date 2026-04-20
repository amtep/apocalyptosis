use bevy::prelude::*;
use rand::{make_rng, rngs::StdRng};

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_rng);
    }
}

#[derive(Resource)]
pub struct RandomSource(pub StdRng);

fn setup_rng(mut commands: Commands) {
    commands.insert_resource(RandomSource(make_rng()));
}
