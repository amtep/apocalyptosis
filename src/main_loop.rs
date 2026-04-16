use bevy::{audio::AudioPlugin, prelude::*};

use crate::{
    funds::setup_funds,
    regions::RegionsPlugin,
    text::LocalizedTextPlugin,
    time::{advance_game_time, setup_game_time},
    ui::setup_ui,
};

pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<AudioPlugin>())
        .add_plugins(RegionsPlugin)
        .add_plugins(LocalizedTextPlugin)
        .add_systems(
            Startup,
            (setup_ui, (setup_game_time, setup_funds).after(setup_ui)),
        )
        .add_systems(FixedUpdate, advance_game_time)
        .run();
}
