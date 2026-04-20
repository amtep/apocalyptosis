use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use rand::RngExt;
use serde_derive::Deserialize;

use crate::{
    funds::{Expense, ExpenseCategory, FundsAmount},
    regions::Region,
    rng::RandomSource,
    state::{GameState, MainSetupSet},
};

const BASES_ASSET_PATH: &str = "data/defines.base.toml";

pub struct BasesPlugin;

impl Plugin for BasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<BaseTypesAsset>::new(&["base.toml"]))
            .add_systems(OnEnter(GameState::Load), setup_load)
            .add_systems(
                OnEnter(GameState::Main),
                setup_main.in_set(MainSetupSet::Late),
            );
    }
}

#[derive(Deserialize, Asset, TypePath)]
pub struct BaseTypesAsset(HashMap<String, BaseTypeSettings>);

#[derive(Resource)]
pub struct BaseTypesHandle(Handle<BaseTypesAsset>);

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub struct BaseTypeSettings {
    pub people: isize,
    pub cost_per_day: FundsAmount,
    pub initial_cost: FundsAmount,
}

fn setup_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BaseTypesHandle(asset_server.load(BASES_ASSET_PATH)));
}

/// The `String` is the key for the base type in the BaseTypes asset.
#[derive(Component)]
pub struct BaseType {
    pub name: String,
    pub settings: BaseTypeSettings,
}

#[derive(Event)]
pub struct SpawnBaseEvent {
    pub region: Entity,
    pub base_type: Entity,
}

fn setup_main(
    mut commands: Commands,
    base_types_handle: Res<BaseTypesHandle>,
    base_types_asset: Res<Assets<BaseTypesAsset>>,
    mut random_source: ResMut<RandomSource>,
    regions: Query<Entity, With<Region>>,
) {
    info!("Creating starting base");
    let i = random_source.0.random_range(0..regions.count());
    let region = regions.iter().nth(i).unwrap();

    let base_types = &base_types_asset.get(base_types_handle.0.id()).unwrap().0;
    let apartment = base_types.get_key_value("apartment").unwrap();
    let apartment = commands
        .spawn((
            BaseType {
                name: apartment.0.clone(),
                settings: *apartment.1,
            },
            Expense(apartment.1.cost_per_day, ExpenseCategory::Bases),
        ))
        .id();
    commands.entity(region).add_child(apartment);
    commands.trigger(SpawnBaseEvent {
        region,
        base_type: apartment,
    });
}
