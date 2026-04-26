use std::{
    fs::{File, create_dir_all, read_dir},
    io::Write,
    path::PathBuf,
};

use bevy::prelude::*;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use moonshine_save::save::{SaveWorld, TriggerSave, save_on_default_event};
use serde::{Deserialize, Serialize};
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
    ui::{FontHandle, save_load::warn_no_save},
};

const SEPARATOR: &[u8] = b"\n\nAPOCALYPTOSIS\n";

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

#[derive(Serialize, Deserialize)]
pub struct SaveMetadata {
    pub save_timestamp: DateTime<Utc>,
}

#[derive(Resource, Deref, Reflect)]
#[reflect(Resource)]
pub struct Campaign(usize);

#[derive(Resource, Deref, DerefMut)]
struct AutosaveTimer(Timer);

#[derive(Error, Debug)]
pub enum SaveLoadError {
    #[error("could not locate user home for project folder")]
    ProjectDirFailed,
    #[error("could not create savegame folder {0}: {1}")]
    CreateDirError(PathBuf, std::io::Error),
    #[error("could not open savegame folder {0}: {1}")]
    ReadDirError(PathBuf, std::io::Error),
    #[error("could not read savegame folder {0}: {1}")]
    ReadEntryError(PathBuf, std::io::Error),
    #[error("could not create save file {0}: {1}")]
    CreateSaveError(PathBuf, std::io::Error),
    #[error("could not write save file {0}: {1}")]
    WriteSaveError(PathBuf, std::io::Error),
    #[error("could not read save file {0}: {1}")]
    ReadSaveError(PathBuf, std::io::Error),
}

fn save_inner(mut commands: Commands, index: usize) -> Result<(), SaveLoadError> {
    if let Some(pd) = ProjectDirs::from(
        PROJECT_DIR_QUALIFIER,
        PROJECT_DIR_ORGANIZATION,
        PROJECT_DIR_APPLICATION,
    ) {
        let path = pd
            .data_dir()
            .join(format!("saves/{index}.apocalyptosis.save"));
        info!("Saving to {}", path.display());
        let mut file =
            File::create(&path).map_err(|e| SaveLoadError::CreateSaveError(path.clone(), e))?;
        let metadata = SaveMetadata {
            save_timestamp: Utc::now(),
        };
        file.write_all(ron::to_string(&metadata).unwrap().as_bytes())
            .map_err(|e| SaveLoadError::WriteSaveError(path.clone(), e))?;
        file.write_all(SEPARATOR)
            .map_err(|e| SaveLoadError::WriteSaveError(path.clone(), e))?;
        let event = SaveWorld::default_into_stream(file)
            .include_resource::<Campaign>()
            .include_resource::<Funds>()
            .include_resource::<IntelligenceSuspicion>()
            .include_resource::<ScientificSuspicion>()
            .include_resource::<GameDate>();
        commands.trigger_save(event);
        Ok(())
    } else {
        Err(SaveLoadError::ProjectDirFailed)
    }
}

fn save(mut commands: Commands, campaign: Option<Res<Campaign>>, font: Res<FontHandle>) {
    let index = if let Some(index) = campaign {
        **index
    } else {
        match calc_new_campaign_index() {
            Ok(index) => {
                commands.insert_resource(Campaign(index));
                index
            }
            Err(e) => {
                error!("Save error! could not determine campaign index: {e}");
                warn_no_save(commands.reborrow(), font.0.clone());
                return;
            }
        }
    };
    if let Err(e) = save_inner(commands.reborrow(), index) {
        error!("Save error! {e}");
        warn_no_save(commands.reborrow(), font.0.clone());
    }
}

fn autosave(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut timer: ResMut<AutosaveTimer>,
    campaign: Option<Res<Campaign>>,
    font: Res<FontHandle>,
) {
    if timer.tick(time.delta()).just_finished() {
        save(commands.reborrow(), campaign, font);
    }
}

fn listen_save_keys(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    campaign: Option<Res<Campaign>>,
    font: Res<FontHandle>,
) {
    if keys.just_pressed(KeyCode::F5) {
        save(commands.reborrow(), campaign, font);
    }
}

/// Examine the savefile filenames to find a new number to save under.
fn calc_new_campaign_index() -> Result<usize, SaveLoadError> {
    if let Some(pd) = ProjectDirs::from(
        PROJECT_DIR_QUALIFIER,
        PROJECT_DIR_ORGANIZATION,
        PROJECT_DIR_APPLICATION,
    ) {
        let mut max_campaign_index = 0;
        let save_dir = pd.data_dir().join("saves");
        create_dir_all(&save_dir)
            .map_err(|e| SaveLoadError::CreateDirError(save_dir.to_owned(), e))?;
        for entry in
            read_dir(&save_dir).map_err(|e| SaveLoadError::ReadDirError(save_dir.to_owned(), e))?
        {
            let entry = entry.map_err(|e| SaveLoadError::ReadEntryError(save_dir.to_owned(), e))?;
            // Parse the leading number in the filename
            if let Some(Ok(index)) = entry
                .file_name()
                .to_string_lossy()
                .split(&['.', '-'])
                .next()
                .map(|number| number.parse())
                && index > max_campaign_index
            {
                max_campaign_index = index;
            }
        }
        Ok(max_campaign_index + 1)
    } else {
        Err(SaveLoadError::ProjectDirFailed)
    }
}

pub fn scan_saved_games() -> Result<Vec<(Campaign, SaveMetadata, Vec<u8>)>, SaveLoadError> {
    if let Some(pd) = ProjectDirs::from(
        PROJECT_DIR_QUALIFIER,
        PROJECT_DIR_ORGANIZATION,
        PROJECT_DIR_APPLICATION,
    ) {
        let mut v = Vec::default();
        let save_dir = pd.data_dir().join("saves");
        create_dir_all(&save_dir)
            .map_err(|e| SaveLoadError::CreateDirError(save_dir.to_owned(), e))?;
        for entry in
            read_dir(&save_dir).map_err(|e| SaveLoadError::ReadDirError(save_dir.to_owned(), e))?
        {
            let entry = entry.map_err(|e| SaveLoadError::ReadEntryError(save_dir.to_owned(), e))?;
            // Parse the leading number in the filename
            if let Some(Ok(index)) = entry
                .file_name()
                .to_string_lossy()
                .split(&['.', '-'])
                .next()
                .map(|number| number.parse())
            {
                let Ok(bytes) = std::fs::read(entry.path()).map_err(|e| {
                    let e = SaveLoadError::ReadSaveError(save_dir.to_owned(), e);
                    error!("Skipping save file: {e}");
                }) else {
                    continue;
                };
                let Some(p) = bytes
                    .windows(SEPARATOR.len())
                    .position(|window| window == SEPARATOR)
                else {
                    error!("Savefile without metadata: {}", entry.path().display());
                    continue;
                };
                let (metadata, content) = (&bytes[..p], &bytes[p + SEPARATOR.len()..]);
                let Ok(metadata) = ron::de::from_bytes(metadata) else {
                    error!("Savefile with invalid metadata: {}", entry.path().display());
                    continue;
                };
                v.push((Campaign(index), metadata, content.to_owned()));
            }
        }
        Ok(v)
    } else {
        Err(SaveLoadError::ProjectDirFailed)
    }
}
