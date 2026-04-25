use bevy::{prelude::*, window::WindowMode};
use pyri_tooltip::TooltipPlugin;

pub fn main_loop() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // During development: use the assets from the source dir.
                    // This is the default, but here the path is set regardless
                    // of the current directory.
                    file_path: format!("{}/assets", env!("CARGO_MANIFEST_DIR")),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..default()
                    }),
                    ..default()
                }),
            TooltipPlugin::default(),
            crate::state::plugin,
            crate::text::plugin,
            crate::regions::plugin,
            crate::bases::plugin,
            crate::rng::plugin,
            crate::funds::plugin,
            crate::ui::plugin,
            crate::time::plugin,
            crate::followers::plugin,
            crate::suspicion::plugin,
            crate::main_menu::plugin,
        ))
        .run();
}
