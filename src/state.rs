use bevy::{asset::LoadedFolder, prelude::*};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::Load), load_setup)
            .add_systems(Update, load_update.run_if(in_state(GameState::Load)))
            .add_systems(OnExit(GameState::Load), load_cleanup)
            .configure_sets(
                OnEnter(GameState::Main),
                (
                    MainSetupSet::Default,
                    MainSetupSet::Regions,
                    MainSetupSet::Ui,
                    MainSetupSet::Bases,
                    MainSetupSet::Followers,
                    MainSetupSet::Late,
                )
                    .chain(),
            );
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Load,
    Main,
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MainSetupSet {
    Default,
    Regions,
    Bases,
    Followers,
    Ui,
    Late,
}

#[derive(Resource)]
struct LoadHandle(Handle<LoadedFolder>);

fn load_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entered load game state");
    commands.insert_resource(LoadHandle(asset_server.load_folder(".")));
    commands.spawn(BackgroundColor(Color::BLACK));
}

fn load_update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    load_handle: Res<LoadHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.is_loaded(load_handle.0.id()) {
        commands.remove_resource::<LoadHandle>();
        next_state.set(GameState::Main);
    }
}

fn load_cleanup(mut commands: Commands, background: Single<(Entity, &BackgroundColor)>) {
    info!("exited load game state");
    commands.entity(background.0).despawn();
}
