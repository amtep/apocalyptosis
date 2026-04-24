use bevy::prelude::*;
use rand::RngExt;
use rand_distr::Poisson;

use crate::{
    rng::RandomSource,
    state::{GameState, MainSetupSet},
    time::GameDate,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        setup_main.in_set(MainSetupSet::Default),
    )
    .add_systems(
        FixedUpdate,
        update_suspicion
            .run_if(resource_exists_and_changed::<GameDate>.and(in_state(GameState::Main))),
    );
}

// global
#[derive(Resource, Default)]
pub struct IntelligenceSuspicion(pub u32);

#[derive(Resource, Default)]
pub struct ScientificSuspicion(pub u32);

// regional
#[derive(Component, Default)]
pub struct PoliceSuspicion(pub u32);

#[derive(Component, Default)]
pub struct MediaSuspicion(pub u32);

fn setup_main(mut commands: Commands) {
    commands.init_resource::<IntelligenceSuspicion>();
    commands.init_resource::<ScientificSuspicion>();
}

fn update_suspicion(
    mut intel_suspicion: ResMut<IntelligenceSuspicion>,
    mut scien_suspicion: ResMut<ScientificSuspicion>,
    mut random: ResMut<RandomSource>,
) {
    intel_suspicion.0 += random.0.sample(Poisson::new(1.0).unwrap()) as u32;
    scien_suspicion.0 += random.0.sample(Poisson::new(1.0).unwrap()) as u32;
}
