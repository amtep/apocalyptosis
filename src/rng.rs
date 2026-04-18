use bevy::prelude::*;
use rand::{make_rng, rngs::StdRng};

#[derive(Resource)]
pub struct RandomSource(pub StdRng);

pub fn setup_rng(mut commands: Commands) {
    commands.insert_resource(RandomSource(make_rng()));
}
