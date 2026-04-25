use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use rand::RngExt;
use serde_derive::Deserialize;

use crate::{
    funds::{Expense, ExpenseCategory, FundsAmount},
    main_menu::NewGame,
    regions::BasePlot,
    rng::RandomSource,
    state::{GameState, MainSetupSet},
};

const BASETYPES_ASSET_PATH: &str = "data/define.basetypes.toml";

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<BasetypesAsset>::new(&["basetypes.toml"]))
        .add_systems(OnEnter(GameState::Load), setup_load)
        .add_systems(
            OnEnter(GameState::Main),
            new_game
                .run_if(resource_exists::<NewGame>)
                .in_set(MainSetupSet::Bases),
        );
}

#[derive(Deserialize, Asset, TypePath)]
struct BasetypesAsset(HashMap<String, BasetypeSettings>);

#[derive(Resource)]
struct BasetypesHandle(Handle<BasetypesAsset>);

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub struct BasetypeSettings {
    pub people: isize,
    pub cost_per_day: FundsAmount,
    pub initial_cost: FundsAmount,
    pub police_suspicion: u32,
    pub media_suspicion: u32,
}

fn setup_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BasetypesHandle(asset_server.load(BASETYPES_ASSET_PATH)));
}

/// A marker component for bases in the game state.
#[derive(Component)]
pub struct Base;

/// The `String` is the key for the base type in the `Basetypes` asset.
#[derive(Component)]
pub struct Basetype {
    pub name: String,
    pub settings: BasetypeSettings,
}

fn new_game(
    mut commands: Commands,
    base_types_handle: Res<BasetypesHandle>,
    base_types_asset: Res<Assets<BasetypesAsset>>,
    mut random_source: ResMut<RandomSource>,
    base_plots: Query<Entity, With<BasePlot>>,
) {
    info!("Creating starting base");
    let i = random_source.0.random_range(0..base_plots.count());
    let base_plot = base_plots.iter().nth(i).unwrap();

    let base_types = &base_types_asset.get(base_types_handle.0.id()).unwrap().0;
    // TODO: don't hardcode this string
    let apartment = base_types.get_key_value("apartment").unwrap();
    commands.entity(base_plot).with_child((
        Base,
        Basetype {
            name: apartment.0.clone(),
            settings: *apartment.1,
        },
        Expense(apartment.1.cost_per_day, ExpenseCategory::Bases),
    ));
}
