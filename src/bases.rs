use std::collections::HashMap;

use bevy::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;
use rand::RngExt;
use serde_derive::Deserialize;

use crate::{
    constants::ui::{FONT_PATH, MENU_BACKGROUND},
    funds::{Expense, ExpenseCategory, FundsAmount},
    main_loop::NewGame,
    regions::RegionUi,
    rng::RandomSource,
    text::TextKey,
};

const BASETYPES_ASSET_PATH: &str = "data/basetypes.toml";

pub struct BasesPlugin;

impl Plugin for BasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<BasetypesAsset>::new(&["toml"]))
            .add_systems(Startup, startup);
    }
}

#[derive(Deserialize, Asset, TypePath)]
struct BasetypesAsset(HashMap<String, BasetypeSettings>);

#[derive(Deserialize)]
#[allow(dead_code)] // TODO
#[serde(rename_all = "kebab-case")]
struct BasetypeSettings {
    people: isize,
    cost_per_day: FundsAmount,
    initial_cost: FundsAmount,
}

#[derive(Resource)]
pub struct BasetypesHandle(Handle<BasetypesAsset>);

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BasetypesHandle(
        asset_server.load::<BasetypesAsset>(BASETYPES_ASSET_PATH),
    ));
    commands.add_observer(spawn_base);
    commands.add_observer(new_game_bases);
}

/// Marker for a base in a region.
/// The base entity will be a child of the region entity.
#[derive(Component)]
pub struct Base;

/// The `String` is the key for the base type in the `Basetypes` asset.
#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct Basetype(pub String);

#[derive(Event)]
pub struct SpawnBaseDirective {
    base_type: String,
    region: Entity,
}

fn spawn_base(
    event: On<SpawnBaseDirective>,
    mut commands: Commands,
    assets: Res<Assets<BasetypesAsset>>,
    base_types: Res<BasetypesHandle>,
    asset_server: Res<AssetServer>,
) {
    let Some(base_types) = assets.get(&base_types.0) else {
        warn!("Base types not loaded yet; not spawning base.");
        return;
    };
    let Some(settings) = base_types.0.get(&event.base_type) else {
        warn!(
            "Base type {} not known; not spawning base.",
            &event.base_type
        );
        return;
    };

    let font = asset_server.load(FONT_PATH);
    commands
        .spawn((
            Base,
            Basetype(event.base_type.clone()),
            Expense(settings.cost_per_day, ExpenseCategory::Bases),
            ChildOf(event.region),
            Node {
                border: UiRect::all(px(1)),
                padding: UiRect::horizontal(px(2)),
                justify_content: JustifyContent::End,
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(MENU_BACKGROUND.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextKey(format!("basetype-{}", &event.base_type)),
                TextFont {
                    font: font.clone(),
                    ..default()
                },
            ));
        });
}

// TODO: have a proper Region component and entity instead of relying on RegionUi
fn new_game_bases(
    _: On<NewGame>,
    mut commands: Commands,
    mut random_source: ResMut<RandomSource>,
    regions: Query<Entity, With<RegionUi>>,
) {
    info!("Creating starting base");
    let i = random_source.0.random_range(0..regions.count());
    let region_e = regions.iter().nth(i).unwrap();
    let event = SpawnBaseDirective {
        base_type: "apartment".to_string(),
        region: region_e,
    };
    commands.trigger(event);
}
