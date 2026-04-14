use bevy::{audio::AudioPlugin, prelude::*};

use crate::{regions::RegionsPlugin, text::LocalizedTextPlugin, ui::setup_ui};
pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<AudioPlugin>())
        .add_plugins(RegionsPlugin)
        .add_plugins(LocalizedTextPlugin)
        .add_systems(Startup, setup_ui)
        .run();
}
