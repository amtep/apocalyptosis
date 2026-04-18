use std::collections::HashMap;

use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};
use bevy_common_assets::toml::TomlAssetPlugin;
use rand::RngExt;
use serde_derive::Deserialize;

use crate::{
    constants::FONT_PATH,
    funds::{Expense, ExpenseCategory},
    main_loop::NewGame,
    regions::RegionUi,
    rng::RandomSource,
    text::TextKey,
};

const BASES_ASSET_PATH: &str = "data/bases.toml";

pub struct BasesPlugin;

impl Plugin for BasesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TomlAssetPlugin::<BasesAsset>::new(&["toml"]))
            .add_systems(Startup, setup_basetypes);
    }
}

#[derive(Deserialize, Asset, TypePath)]
pub struct BasesAsset(HashMap<String, BaseTypeSettings>);

#[derive(Deserialize)]
#[allow(dead_code)] // TODO
#[serde(rename_all = "kebab-case")]
struct BaseTypeSettings {
    people: isize,
    cost_per_day: i64,
    initial_cost: i64,
}

#[derive(Resource)]
pub struct BaseTypesResource(Handle<BasesAsset>);

fn setup_basetypes(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(BaseTypesResource(
        asset_server.load::<BasesAsset>(BASES_ASSET_PATH),
    ));
    commands.add_observer(spawn_base);
    commands.add_observer(new_game_bases);
}

/// Marker for a base in a region.
/// The base entity will be a child of the region entity.
#[derive(Component)]
pub struct Base;

/// The `String` is the key for the base type in the BaseTypes asset.
#[derive(Component)]
#[allow(dead_code)] // TODO
pub struct BaseType(pub String);

#[derive(Event)]
pub struct SpawnBaseDirective {
    base_type: String,
    region: Entity,
}

fn spawn_base(
    event: On<SpawnBaseDirective>,
    mut commands: Commands,
    assets: Res<Assets<BasesAsset>>,
    base_types: Res<BaseTypesResource>,
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
            BaseType(event.base_type.clone()),
            Expense(settings.cost_per_day, ExpenseCategory::Bases),
            ChildOf(event.region),
            Node {
                border: UiRect::all(px(1)),
                padding: UiRect::horizontal(px(2)),
                justify_content: JustifyContent::End,
                ..default()
            },
            BorderColor::all(WHITE),
            BackgroundColor(BLACK.into()),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text(String::new()),
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
