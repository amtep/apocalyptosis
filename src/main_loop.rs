use bevy::{audio::AudioPlugin, prelude::*};

use crate::regions::RegionsPlugin;
use crate::ui::setup_ui;

pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<AudioPlugin>())
        .add_plugins(RegionsPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}
