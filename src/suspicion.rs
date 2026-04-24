use bevy::prelude::*;
use rand::RngExt;
use rand_distr::Poisson;

use crate::{
    rng::RandomSource,
    state::{GameState, MainSetupSet},
    time::GameDateChangedEvent,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Main),
        setup_main.in_set(MainSetupSet::Late),
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

#[derive(Event)]
pub struct SuspicionsChangedEvent;

fn setup_main(mut commands: Commands) {
    commands.init_resource::<IntelligenceSuspicion>();
    commands.init_resource::<ScientificSuspicion>();
    commands.add_observer(on_game_date_inc_suspicion);
}

fn on_game_date_inc_suspicion(
    _: On<GameDateChangedEvent>,
    mut commands: Commands,
    mut intel_suspicion: ResMut<IntelligenceSuspicion>,
    mut scien_suspicion: ResMut<ScientificSuspicion>,
    mut random: ResMut<RandomSource>,
) {
    intel_suspicion.0 += random.0.sample(Poisson::new(1.0).unwrap()) as u32;
    scien_suspicion.0 += random.0.sample(Poisson::new(1.0).unwrap()) as u32;
    commands.trigger(SuspicionsChangedEvent);
}
