use std::{
    fs::{create_dir_all, read_dir},
    path::PathBuf,
};

use bevy::prelude::*;
use directories::ProjectDirs;
use moonshine_save::save::{SaveWorld, TriggerSave, save_on_default_event};
use thiserror::Error;

use crate::{
    constants::{
        AUTOSAVE_INTERVAL,
        files::{PROJECT_DIR_APPLICATION, PROJECT_DIR_ORGANIZATION, PROJECT_DIR_QUALIFIER},
    },
    funds::Funds,
    state::GameState,
    suspicion::{IntelligenceSuspicion, ScientificSuspicion},
    time::GameDate,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (autosave, listen_save_keys).run_if(in_state(GameState::Main)),
    )
    .insert_resource(AutosaveTimer(Timer::new(
        AUTOSAVE_INTERVAL,
        TimerMode::Repeating,
    )))
    .add_observer(save_on_default_event);
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct Campaign(pub usize);

#[derive(Resource, Deref, DerefMut)]
struct AutosaveTimer(Timer);

#[derive(Error, Debug)]
enum SaveError {
    #[error("could not locate user home for project folder")]
    ProjectDirFailed,
    #[error("could not create savegame folder {0}")]
    CreateDirError(PathBuf, std::io::Error),
    #[error("could not open savegame folder {0}")]
    ReadDirError(PathBuf, std::io::Error),
    #[error("could not read savegame folder {0}")]
    ReadEntryError(PathBuf, std::io::Error),
}

fn save(mut commands: Commands, campaign: Option<Res<Campaign>>) {
    let index = if let Some(index) = campaign {
        **index
    } else {
        match calc_new_campaign_index() {
            Ok(index) => {
                commands.insert_resource(Campaign(index));
                index
            }
            Err(e) => {
                error!("could not determine campaign index: {e} {e:?}");
                // TODO: open a popup warning the user.
                return;
            }
        }
    };
    if let Some(pd) = ProjectDirs::from(
        PROJECT_DIR_QUALIFIER,
        PROJECT_DIR_ORGANIZATION,
        PROJECT_DIR_APPLICATION,
    ) {
        let path = pd.data_dir().join(format!("saves/{index}.ap.save"));
        info!("Saving to {}", path.display());
        let event = SaveWorld::default_into_file(path)
            .include_resource::<Campaign>()
            .include_resource::<Funds>()
            .include_resource::<IntelligenceSuspicion>()
            .include_resource::<ScientificSuspicion>()
            .include_resource::<GameDate>();
        commands.trigger_save(event);
    } else {
        error!("{}", SaveError::ProjectDirFailed);
        // TODO: open a popup warning the user.
        return;
    }
}

fn autosave(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut timer: ResMut<AutosaveTimer>,
    campaign: Option<Res<Campaign>>,
) {
    if timer.tick(time.delta()).just_finished() {
        save(commands.reborrow(), campaign);
    }
}

fn listen_save_keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    campaign: Option<Res<Campaign>>,
) {
    if keys.just_pressed(KeyCode::F5) {
        save(commands.reborrow(), campaign);
    }
}

fn calc_new_campaign_index() -> Result<usize, SaveError> {
    if let Some(pd) = ProjectDirs::from(
        PROJECT_DIR_QUALIFIER,
        PROJECT_DIR_ORGANIZATION,
        PROJECT_DIR_APPLICATION,
    ) {
        let mut max_campaign_index = 0;
        let save_dir = pd.data_dir().join("saves");
        create_dir_all(&save_dir).map_err(|e| SaveError::CreateDirError(save_dir.to_owned(), e))?;
        for entry in
            read_dir(&save_dir).map_err(|e| SaveError::ReadDirError(save_dir.to_owned(), e))?
        {
            let entry = entry.map_err(|e| SaveError::ReadEntryError(save_dir.to_owned(), e))?;
            // Parse the leading number in the filename
            if let Some(Ok(index)) = entry
                .file_name()
                .to_string_lossy()
                .split(&['.', '-'])
                .nth(0)
                .map(|number| number.parse())
                && index > max_campaign_index
            {
                max_campaign_index = index;
            }
        }
        Ok(max_campaign_index + 1)
    } else {
        Err(SaveError::ProjectDirFailed)
    }
}
