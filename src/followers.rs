use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use rand::RngExt;
use serde::Deserialize;

use crate::{
    bases::Base,
    funds::{Expense, ExpenseCategory, FundsAmount},
    rng::RandomSource,
    state::{GameState, MainSetupSet},
};

const FOLLOWERS_ASSET_PATH: &str = "data/define.followers.toml";

pub fn plugin(app: &mut App) {
    app.add_plugins(TomlAssetPlugin::<FollowersAsset>::new(&["followers.toml"]))
        .add_systems(OnEnter(GameState::Load), setup_load)
        .add_systems(
            OnEnter(GameState::Main),
            new_spawn_follower.in_set(MainSetupSet::Followers),
        );
}

#[derive(Deserialize, Asset, TypePath)]
struct FollowersAsset(HashMap<String, GeneralFollowerSettings>);

#[derive(Resource)]
struct FollowersHandle(Handle<FollowersAsset>);

/// These are the general settings for all follower types.
/// Once there are also specific follower settings, there
/// will need to be an enum to distinguish them.
#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
struct GeneralFollowerSettings {
    cost_per_day: FundsAmount,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Follower {
    Priest,
    Goon,
    Minion,
}

fn setup_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FollowersHandle(asset_server.load(FOLLOWERS_ASSET_PATH)));
}

/// Create the starting priest for the cult.
fn new_spawn_follower(
    mut commands: Commands,
    bases: Query<Entity, With<Base>>,
    followers_handle: Res<FollowersHandle>,
    followers_asset: Res<Assets<FollowersAsset>>,
    mut random_source: ResMut<RandomSource>,
) {
    info!("Creating starting priest");
    let i = random_source.0.random_range(0..bases.count());
    let base = bases.iter().nth(i).unwrap();

    // SAFETY: this will be called after the Load state, where everything is loaded.
    let settings = &followers_asset.get(followers_handle.0.id()).unwrap();
    let cost = settings
        .0
        .get("general")
        .map(|v| v.cost_per_day)
        .unwrap_or(0);

    // Generally we should check whether the base has room
    // for another follower, but this is a new game and it
    // will be empty.

    // INFO: need to ensure the children relationship target is updated BEFORE the follower.
    commands
        .spawn((ChildOf(base), Expense(cost, ExpenseCategory::Followers)))
        .insert(Follower::Priest);
}
