use bevy::prelude::*;
use pyri_tooltip::TooltipPlugin;

use crate::{
    bases::BasesPlugin, date::DatePlugin, funds::FundsPlugin, locales::LocalesPlugin,
    regions::RegionsPlugin, rng::RngPlugin, state::StatePlugin, ui::UiPlugin,
};

pub fn main_loop() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StatePlugin)
        .add_plugins(LocalesPlugin)
        .add_plugins(RegionsPlugin)
        .add_plugins(BasesPlugin)
        .add_plugins(RngPlugin)
        .add_plugins(FundsPlugin)
        .add_plugins(DatePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(TooltipPlugin::default())
        .run();
}
