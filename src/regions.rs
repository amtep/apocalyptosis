use std::collections::{HashMap, HashSet};

use bevy::{asset::AsAssetId, prelude::*};
use bevy_common_assets::toml::TomlAssetPlugin;
use serde::Deserialize;

use crate::{text::TextKey, ui::MapUi};

const REGIONS_ASSET_PATH: &str = "data/regions.toml";

pub struct RegionsPlugin;

impl Plugin for RegionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<RegionsAsset>::new(&["toml"]))
            .add_systems(Startup, setup_regions)
            .add_systems(Update, watch_regions);
    }
}

#[derive(Deserialize, Asset, TypePath)]
pub struct RegionsAsset(HashMap<String, RegionSettings>);

#[derive(Deserialize)]
struct RegionSettings {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Regions(Handle<RegionsAsset>);

impl AsAssetId for Regions {
    type Asset = RegionsAsset;
    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}

#[derive(Component)]
struct RegionUi(String);

fn setup_regions(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Regions(asset_server.load(REGIONS_ASSET_PATH)));
}

fn watch_regions(
    mut commands: Commands,
    assets: Res<Assets<RegionsAsset>>,
    regions: Single<&Regions, AssetChanged<Regions>>,
    mut q_ui: Query<(Entity, &mut Text, &mut Node, &RegionUi)>,
    mapui: Single<Entity, With<MapUi>>,
) {
    info!("regions changed");
    if let Some(settings) = assets.get(&regions.0) {
        let mut seen: HashSet<&str> = HashSet::default();
        for (region_e, mut text, mut node, region) in &mut q_ui {
            if let Some(region_settings) = settings.0.get(&region.0) {
                seen.insert(&region.0);
                node.left = percent(region_settings.x);
                node.top = percent(region_settings.y);
                text.0 = region.0.clone();
            } else {
                commands.entity(region_e).despawn();
            }
        }
        for (key, region_settings) in &settings.0 {
            if !seen.contains(&key.as_ref()) {
                commands.spawn((
                    Text::new("".to_string()),
                    Node {
                        position_type: PositionType::Absolute,
                        left: percent(region_settings.x),
                        top: percent(region_settings.y),
                        ..default()
                    },
                    TextKey(key.clone()),
                    RegionUi(key.clone()),
                    ChildOf(*mapui),
                ));
            }
        }
    }
}
