use bevy::prelude::*;
use pyri_tooltip::TooltipPlugin;

use crate::{
    bases::BasesPlugin,
    funds::setup_funds,
    regions::RegionsPlugin,
    rng::setup_rng,
    text::LocalizedTextPlugin,
    time::{advance_game_time, listen_speed_buttons, listen_speed_keys, setup_game_time},
    ui::{setup_ui, update_button_colors, update_funds_displays},
};

#[derive(Event)]
pub struct NewGame;

pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TooltipPlugin::default())
        .add_plugins(RegionsPlugin)
        .add_plugins(BasesPlugin)
        .add_plugins(LocalizedTextPlugin)
        .add_systems(
            Startup,
            (
                setup_rng,
                setup_ui,
                (setup_game_time, setup_funds).after(setup_ui),
            ),
        )
        .add_systems(
            Update,
            (
                update_button_colors,
                listen_speed_buttons,
                listen_speed_keys,
                update_funds_displays,
            ),
        )
        .add_systems(FixedUpdate, advance_game_time)
        .run();
}
