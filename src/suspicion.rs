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
        setup_main.in_set(MainSetupSet::Default),
    );
}

#[derive(Resource, Default)]
pub struct Suspicion(pub u32);

fn setup_main(mut commands: Commands) {
    commands.init_resource::<Suspicion>();
    commands.add_observer(on_game_date_inc_suspicion);
}

fn on_game_date_inc_suspicion(
    _: On<GameDateChangedEvent>,
    mut suspicion: ResMut<Suspicion>,
    mut random: ResMut<RandomSource>,
) {
    suspicion.0 += random.0.sample(Poisson::new(1.0).unwrap()) as u32;
}
