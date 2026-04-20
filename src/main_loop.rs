use bevy::prelude::*;
use pyri_tooltip::TooltipPlugin;

use crate::{
    bases::BasesPlugin, funds::FundsPlugin, regions::RegionsPlugin, rng::RngPlugin,
    state::StatePlugin, text::TextPlugin, time::TimePlugin, ui::UiPlugin,
};

pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            // During development: use the assets from the source dir.
            // This is the default, but here the path is set regardless
            // of the current directory.
            file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
            ..default()
        }))
        .add_plugins(TooltipPlugin::default())
        .add_plugins(StatePlugin)
        .add_plugins(TextPlugin)
        .add_plugins(RegionsPlugin)
        .add_plugins(BasesPlugin)
        .add_plugins(RngPlugin)
        .add_plugins(FundsPlugin)
        .add_plugins(TimePlugin)
        .add_plugins(UiPlugin)
        .run();
}
