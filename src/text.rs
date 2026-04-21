use std::string::FromUtf8Error;
use std::sync::Arc;

use bevy::asset::RecursiveDependencyLoadState;
use bevy::asset::{AssetLoader, LoadContext, LoadedFolder, io::Reader};
use bevy::prelude::*;
use fluent::{FluentArgs, FluentResource, concurrent::FluentBundle};
use fluent_datetime::BundleExt;
use line_numbers::LinePositions;
use thiserror::Error;
use unic_langid::langid;

use crate::state::GameState;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Load), setup)
        .add_systems(Update, update.run_if(in_state(GameState::Load)))
        .add_systems(OnExit(GameState::Load), cleanup)
        .add_systems(FixedUpdate, reload.run_if(not(in_state(GameState::Load))))
        .init_asset::<FluentResourceAsset>()
        .register_asset_loader(FluentResourceAssetLoader);
}

/// Add this component to entities that have a Text node that
/// should be derived from this message key.
/// Only works for messages that require no arguments.
#[derive(Component)]
#[require(Text)]
pub struct TextKey(String);

impl TextKey {
    pub fn new<S: Into<String>>(key: S, bundle: &FluentBundleWrapper) -> (Self, Text) {
        let text_key = Self(key.into());
        let text = Text::new(bundle.get(&text_key.0, None));
        (text_key, text)
    }

    pub fn new_no_args<S: Into<String>>(key: S) -> (Self, Text) {
        let text_key = Self(key.into());
        let text = Text::default();
        (text_key, text)
    }

    fn new_args<S: Into<String>>(
        key: S,
        bundle: &FluentBundleWrapper,
        args: &FluentArgs<'_>,
    ) -> (Self, Text) {
        let text_key = Self(key.into());
        let text = Text::new(bundle.get(&text_key.0, Some(args)));
        (text_key, text)
    }

    pub fn get(&self, bundle: &FluentBundleWrapper, args: &FluentArgs<'_>) -> String {
        bundle.get(&self.0, Some(args))
    }
}

#[derive(Debug, Error)]
enum FluentResourceLoaderError {
    #[error("read error: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("invalid utf-8: {0}")]
    InvalidUtf8(#[from] FromUtf8Error),
}

#[derive(TypePath)]
struct FluentResourceAssetLoader;

impl AssetLoader for FluentResourceAssetLoader {
    type Asset = FluentResourceAsset;
    type Settings = ();
    type Error = FluentResourceLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        Ok(FluentResourceAsset(Arc::new(
            match FluentResource::try_new(String::from_utf8(bytes)?) {
                Ok(resource) => resource,
                Err((resource, errs)) => {
                    let line_positions = LinePositions::from(resource.source());
                    for err in errs {
                        let (line_num, column) = line_positions.from_offset(err.pos.start);
                        error!(
                            "{}:{}:{column}: {}",
                            load_context.path(),
                            line_num.display(),
                            err.kind
                        );
                    }
                    resource
                }
            },
        )))
    }

    fn extensions(&self) -> &[&str] {
        &["ftl"]
    }
}

#[derive(Asset, TypePath)]
struct FluentResourceAsset(Arc<FluentResource>);

#[derive(Resource)]
pub struct FluentBundleWrapper(FluentBundle<Arc<FluentResource>>, bool);

impl FluentBundleWrapper {
    pub fn get(&self, key: &str, args: Option<&FluentArgs<'_>>) -> String {
        let Some(msg) = self.0.get_message(key) else {
            error!("no message with key {key} exists");
            return String::new();
        };

        let Some(pattern) = msg.value() else {
            error!("message {key} has no value");
            return String::new();
        };

        let mut errors = vec![];
        let value = self.0.format_pattern(pattern, args, &mut errors);

        for e in errors {
            error!("message {key} formatting error: {e}");
        }

        value.into_owned()
    }
}

#[derive(Resource)]
struct FluentFolder(Handle<LoadedFolder>);

#[derive(Event)]
struct LocalesReloadedEvent;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FluentBundleWrapper(
        FluentBundle::new_concurrent(vec![langid!("en-US")]),
        false,
    ));
    commands.insert_resource(FluentFolder(asset_server.load_folder("text/en-US")));
}

fn new_bundle<'a, I: Iterator<Item = &'a Arc<FluentResource>>>(
    bundle_resource: &FluentBundleWrapper,
    resource_iter: I,
) -> Option<FluentBundle<Arc<FluentResource>>> {
    let mut new_bundle = FluentBundle::new_concurrent(bundle_resource.0.locales.clone());
    if let Err(e) = new_bundle.add_builtins() {
        error!("could not add NUMBER to fluent bundle: {e}");
        return None;
    }
    if let Err(e) = new_bundle.add_datetime_support() {
        error!("could not add DATETIME to fluent bundle: {e}");
        return None;
    }

    for resource in resource_iter {
        if let Err(err) = new_bundle.add_resource(Arc::clone(resource)) {
            for e in err {
                warn!("failed to add to fluent bundle: {e}");
            }
        }
    }

    Some(new_bundle)
}

fn update(
    asset_server: Res<AssetServer>,
    folder: Res<FluentFolder>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    mut bundle: ResMut<FluentBundleWrapper>,
) {
    if !bundle.1
        && matches!(
            asset_server.recursive_dependency_load_state(folder.0.id()),
            RecursiveDependencyLoadState::Loaded
        )
    {
        info!("fluent folder loaded");

        let Some(new_bundle) = new_bundle(
            &bundle,
            fluent_resource_assets.iter().map(|(_, res)| &res.0),
        ) else {
            return;
        };

        bundle.0 = new_bundle;
        bundle.1 = true;
    }
}

fn cleanup(mut messages: ResMut<Messages<AssetEvent<FluentResourceAsset>>>) {
    messages.clear();
}

fn reload(
    mut commands: Commands,
    mut reader: MessageReader<AssetEvent<FluentResourceAsset>>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    mut bundle: ResMut<FluentBundleWrapper>,
) {
    if bundle.1 && !reader.is_empty() {
        info!("fluent bundle reloaded");
        let Some(new_bundle) = new_bundle(
            &bundle,
            fluent_resource_assets.iter().map(|(_, res)| &res.0),
        ) else {
            return;
        };
        bundle.0 = new_bundle;
        reader.clear();
        commands.trigger(LocalesReloadedEvent);
    }
}
