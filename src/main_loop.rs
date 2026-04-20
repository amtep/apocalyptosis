use bevy::prelude::*;
use bevy_aspect_ratio_mask::{AspectRatioMask, AspectRatioPlugin, Resolution};
use pyri_tooltip::TooltipPlugin;

use crate::{
    bases::BasesPlugin,
    constants::ui::{PX_HEIGHT, PX_WIDTH},
    followers::FollowersPlugin,
    funds::FundsPlugin,
    regions::RegionsPlugin,
    rng::RngPlugin,
    state::StatePlugin,
    text::TextPlugin,
    time::TimePlugin,
    ui::UiPlugin,
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
            AspectRatioPlugin {
                resolution: Resolution {
                    width: PX_WIDTH,
                    height: PX_HEIGHT,
                },
                mask: AspectRatioMask::default(),
            },
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
