use bevy::prelude::*;
use pyri_tooltip::TooltipPlugin;

use crate::{
    bases::BasesPlugin, followers::FollowersPlugin, funds::FundsPlugin, regions::RegionsPlugin,
    rng::RngPlugin, state::StatePlugin, text::TextPlugin, time::TimePlugin, ui::UiPlugin,
};

pub fn main_loop() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // During development: use the assets from the source dir.
                // This is the default, but here the path is set regardless
                // of the current directory.
                file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                ..default()
            }),
            TooltipPlugin::default(),
            StatePlugin,
            TextPlugin,
            RegionsPlugin,
            BasesPlugin,
            RngPlugin,
            FundsPlugin,
            TimePlugin,
            FollowersPlugin,
            UiPlugin,
        ))
        .run();
}
