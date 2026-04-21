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

const BASETYPES_ASSET_PATH: &str = "data/define.basetypes.toml";

pub struct BasesPlugin;

impl Plugin for BasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<BasetypesAsset>::new(&["basetypes.toml"]))
            .add_systems(OnEnter(GameState::Load), setup_load)
            .add_systems(
                OnEnter(GameState::Main),
                setup_main.in_set(MainSetupSet::Bases),
            );
    }
}

#[derive(Deserialize, Asset, TypePath)]
struct BasetypesAsset(HashMap<String, BasetypeSettings>);

#[derive(Resource)]
struct BasetypesHandle(Handle<BasetypesAsset>);

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
#[allow(dead_code)] // TODO
struct BasetypeSettings {
    people: isize,
    cost_per_day: FundsAmount,
    initial_cost: FundsAmount,
}

fn setup_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BasetypesHandle(asset_server.load(BASETYPES_ASSET_PATH)));
}

/// A marker component for bases in the game state.
#[derive(Component)]
pub struct Base;

/// The `String` is the key for the base type in the `Basetypes` asset.
#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Basetype {
    pub name: String,
    settings: BasetypeSettings,
}

fn setup_main(
    mut commands: Commands,
    base_types_handle: Res<BasetypesHandle>,
    base_types_asset: Res<Assets<BasetypesAsset>>,
    mut random_source: ResMut<RandomSource>,
    regions: Query<Entity, With<Region>>,
) {
    info!("Creating starting base");
    let i = random_source.0.random_range(0..regions.count());
    let region = regions.iter().nth(i).unwrap();

    let base_types = &base_types_asset.get(base_types_handle.0.id()).unwrap().0;
    let apartment = base_types.get_key_value("apartment").unwrap();
    commands.entity(region).with_child((
        Base,
        Basetype {
            name: apartment.0.clone(),
            settings: *apartment.1,
        },
        Expense(apartment.1.cost_per_day, ExpenseCategory::Bases),
    ));
}
