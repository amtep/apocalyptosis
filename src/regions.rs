use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::Deserialize;

use crate::state::{GameState, MainSetupSet};

const REGIONS_ASSET_PATH: &str = "data/define.regions.toml";

pub struct RegionsPlugin;

impl Plugin for RegionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<RegionsAsset>::new(&["regions.toml"]))
            .add_systems(OnEnter(GameState::Load), setup_load)
            .add_systems(OnExit(GameState::Load), cleanup_load)
            .add_systems(
                OnEnter(GameState::Main),
                setup_main.in_set(MainSetupSet::Regions),
            )
            .add_systems(FixedUpdate, reload.run_if(not(in_state(GameState::Load))));
    }
}

#[derive(Deserialize, Asset, TypePath)]
struct RegionsAsset(HashMap<String, RegionSettings>);

#[derive(Resource)]
struct RegionsHandle(Handle<RegionsAsset>);

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct RegionSettings {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Region {
    pub name: String,
    pub settings: RegionSettings,
}

fn setup_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RegionsHandle(asset_server.load(REGIONS_ASSET_PATH)));
}

fn cleanup_load(mut messages: ResMut<Messages<AssetEvent<RegionsAsset>>>) {
    messages.clear();
}

fn setup_main(
    mut commands: Commands,
    regions_handle: Res<RegionsHandle>,
    regions_asset: Res<Assets<RegionsAsset>>,
) {
    let regions = &regions_asset.get(regions_handle.0.id()).unwrap().0;
    for (name, &settings) in regions.iter() {
        commands.spawn(Region {
            name: name.to_owned(),
            settings,
        });
    }
}

#[derive(Event)]
pub struct RegionsReloadedEvent;

fn reload(
    mut commands: Commands,
    mut reader: MessageReader<AssetEvent<RegionsAsset>>,
    mut regions: Query<&mut Region>,
    regions_handle: Res<RegionsHandle>,
    regions_asset: Res<Assets<RegionsAsset>>,
) {
    if !reader.is_empty() {
        info!("regions reloaded");

        let regions_map = &regions_asset.get(regions_handle.0.id()).unwrap().0;

        for mut region in regions.iter_mut() {
            if let Some(settings) = regions_map.get(&region.name) {
                region.settings = *settings;
            }
        }

        commands.trigger(RegionsReloadedEvent);
        reader.clear();
    }
}
