use bevy::color::palettes::css::{BLACK, WHITE};
use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: vw(100.0),
            height: vh(100.0),
            ..Default::default()
        },
        children![
            (
                Node {
                    width: vw(100.0),
                    height: vh(5.0),
                    border: UiRect {
                        left: vw(0.0),
                        right: vw(0.0),
                        top: vh(0.5),
                        bottom: vh(0.5),
                    },
                    ..Default::default()
                },
                BorderColor::all(WHITE),
                BackgroundColor(BLACK.into()),
            ),
            (
                ImageNode {
                    image: asset_server.load("textures/earth_night.jpg"),
                    image_mode: NodeImageMode::Stretch,
                    ..Default::default()
                },
                Node {
                    width: vw(100.0),
                    height: vh(95.0),
                    ..Default::default()
                },
            )
        ],
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
