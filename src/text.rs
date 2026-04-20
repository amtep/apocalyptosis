use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
    string::FromUtf8Error,
    sync::Arc,
};

use bevy::{
    asset::{
        AsAssetId, AssetLoader, LoadContext, LoadedFolder, RecursiveDependencyLoadState, io::Reader,
    },
    prelude::*,
};
use fluent::{FluentArgs, FluentError, FluentResource, concurrent::FluentBundle};
use fluent_datetime::BundleExt;
use fluent_syntax::ast::Pattern;
use line_numbers::LinePositions;
use thiserror::Error;
use unic_langid::langid;

pub struct LocalizedTextPlugin;

impl Plugin for LocalizedTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text);
        app.add_systems(Update, (watch_fluent_files, update_simple_text_keys));
        app.init_asset::<FluentResourceAsset>();
        app.register_asset_loader(FluentResourceAssetLoader);
    }
}

/// Add this component to entities that have a Text node that
/// should be derived from this message key.
/// They will be automatically updated if the `LocalizedTextPlugin`
/// is loaded.
/// Only works for messages that require no arguments.
#[derive(Component)]
#[require(Text)]
pub struct TextKey(pub String);

impl Display for TextKey {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
pub struct FluentBundleResource(pub FluentBundle<Arc<FluentResource>>);

#[derive(Component)]
struct FluentFolder(Handle<LoadedFolder>);

impl AsAssetId for FluentFolder {
    type Asset = LoadedFolder;

    fn as_asset_id(&self) -> AssetId<Self::Asset> {
        self.0.id()
    }
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(FluentBundleResource(FluentBundle::new_concurrent(vec![
        langid!("en-US"),
    ])));
    commands.spawn(FluentFolder(asset_server.load_folder("text/en-US")));
}

fn watch_fluent_files(
    asset_server: Res<AssetServer>,
    fluent_resource_assets: Res<Assets<FluentResourceAsset>>,
    loaded_folder_assets: Res<Assets<LoadedFolder>>,
    folder: Single<&FluentFolder, AssetChanged<FluentFolder>>,
    mut bundle_res: ResMut<FluentBundleResource>,
) {
    if matches!(
        asset_server.recursive_dependency_load_state(folder.0.id()),
        RecursiveDependencyLoadState::Loaded
    ) {
        info!("fluent folder loaded");
        // Go through the LoadedFolder and add all FluentResource assets
        // to a newly made FluentBundle. This is the only way to account
        // for removed resource files.
        let Some(folder) = loaded_folder_assets.get(folder.0.id()) else {
            error!("fluent folder supposedly loaded, but not available");
            return;
        };
        let mut bundle = FluentBundle::new_concurrent(vec![bundle_res.0.locales[0].clone()]);
        if let Err(e) = bundle.add_builtins() {
            error!("could not add NUMBER to fluent bundle: {e}");
            return;
        }
        if let Err(e) = bundle.add_datetime_support() {
            error!("could not add DATETIME to fluent bundle: {e}");
            return;
        }
        for handle in &folder.handles {
            if let Ok(h) = handle.clone().try_typed::<FluentResourceAsset>()
                && let Some(r) = fluent_resource_assets.get(h.id())
                && let Err(v) = bundle.add_resource(Arc::clone(&r.0))
            {
                for err in v {
                    if let FluentError::Overriding { id, .. } = err {
                        warn!("text key collision: {id}");
                    }
                }
            }
        }
        bundle_res.0 = bundle;
    }
}

fn update_simple_text_keys(q: Query<(&mut Text, &TextKey)>, bundle: Res<FluentBundleResource>) {
    for (mut text, key) in q {
        let new_text = if let Some(pattern) = bundle.get_pattern(&key.0, &key.0) {
            let mut errors = Vec::new();
            let s = bundle.0.format_pattern(pattern, None, &mut errors);
            // Warn only if the text has changed, to avoid warning spam
            if text.0 != s {
                for err in errors {
                    warn!("error evaluating key: {key}: {err}");
                }
            }
            s
        } else {
            Cow::Borrowed(key.0.as_ref())
        };
        // Avoid mutating the component if nothing changed,
        // because components check for mutable derefs.
        if text.0 != new_text {
            text.0 = new_text.into_owned();
        }
    }
}

impl FluentBundleResource {
    pub fn get_pattern<'a>(&'a self, key: &str, old_text: &str) -> Option<&'a Pattern<&'a str>> {
        let Some(msg) = self.0.get_message(key) else {
            // Avoid warning every frame
            if old_text != key {
                warn!("missing text key: {key}");
            }
            return None;
        };
        let Some(value) = msg.value() else {
            // Avoid warning every frame
            if old_text != key {
                warn!("key missing a value: {key}");
            }
            return None;
        };
        Some(value)
    }

    pub fn format_pattern<'a>(
        &'a self,
        key: &str,
        pattern: &'a Pattern<&'a str>,
        args: Option<&FluentArgs>,
        text: &mut String,
    ) {
        let mut errors = Vec::new();
        let new_text = self.0.format_pattern(pattern, args, &mut errors);
        if !errors.is_empty() {
            // Avoid warning every frame
            if text != key {
                for err in errors {
                    warn!("error evaluating key: {key}: {err}");
                }
                *text = key.to_owned(); // fallback
            }
        } else if *text != new_text {
            *text = new_text.into_owned();
        }
    }
}
