use crate::handlers::note_handler::Note;
use directories::ProjectDirs;
use thiserror::Error;

use serde_json;
use std::{fs, path::PathBuf};

// #region public

#[derive(Error, Debug)]
pub enum LoadNotesError {
    #[error("load notes error")]
    IO(#[from] std::io::Error),
    #[error("load uuids error")]
    LoadUUIDSError(#[from] LoadUUIDSError),
    #[error("load note error")]
    LoadNoteError(#[from] LoadNoteError),
}

pub fn load_notes() -> Result<Vec<Note>, LoadNotesError> {
    // Create the app directory if it doesn't exist
    let path = create_project_dir_if_needed()?;

    let uuids = load_all_note_uuids()?;
    let mut notes = Vec::new();

    for uuid in uuids {
        let note: Result<Note, LoadNoteError> = load_note(uuid, Some(path.to_path_buf()));
        notes.push(note?);
    }

    return Ok(notes);
}

#[derive(Error, Debug)]
pub enum LoadNoteError {
    #[error("load note error")]
    IO(#[from] std::io::Error),
    #[error("parse note error")]
    ParseError(#[from] serde_json::Error),
}

pub fn load_note(uuid: String, path: Option<PathBuf>) -> Result<Note, LoadNoteError> {
    let path = match path {
        Some(path) => path,
        None => {
            let path = create_project_dir_if_needed()?;
            path.join(format!("{}.txt", uuid))
        }
    };

    let file_contents = fs::read_to_string(path)?;
    let note: Note = serde_json::from_str(&file_contents)?;

    return Ok(note);
}

pub fn delete_note(uuid: String) -> std::io::Result<()> {
    let project_dirs = get_project_dirs();
    let path = project_dirs.data_dir();
    let file_path = path.join(format!("{}.txt", uuid));

    return fs::remove_file(file_path);
}

#[derive(Error, Debug)]
pub enum SaveNoteError {
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error("parse note error")]
    ParseError(#[from] serde_json::Error),
}

pub fn save_note(note: &Note) -> Result<(), SaveNoteError> {
    // Create the app directory if it doesn't exist
    let path = create_project_dir_if_needed()?;

    // Serialize the note to json
    let json_data = serde_json::to_string(&note)?;

    // Write the json data to a file
    fs::write(path.join(format!("{}.txt", &note.uuid)), json_data)?;
    return Ok(());
}

#[derive(Error, Debug)]
pub enum LoadUUIDSError {
    #[error("data store disconnected")]
    IO(#[from] std::io::Error),
}

pub fn load_all_note_uuids() -> Result<Vec<String>, LoadUUIDSError> {
    // Create the app directory if it doesn't exist
    let path = create_project_dir_if_needed()?;

    let dir_entries = std::fs::read_dir(path)?;
    let mut paths = Vec::new();

    for entry in dir_entries {
        let dir_entry = entry?;
        if dir_entry.path().is_dir() {
            // Skip directories (they are notes
            continue;
        }

        // Parse uuid from file name
        let file_name = dir_entry.file_name().into_string();
        let uuid = match file_name {
            Ok(uuid) => uuid,
            Err(_) => {
                continue;
            }
        };

        paths.push(uuid);
    }

    return Ok(paths);
}

// #endregion public

// #region helper

fn get_project_dirs() -> ProjectDirs {
    let proj_dirs: ProjectDirs = match ProjectDirs::from("com", "Kyle Klus", "Notes") {
        Some(dirs) => dirs,
        None => {
            panic!("Couldn't find project directory");
        }
    };

    return proj_dirs;
}

fn create_project_dir_if_needed() -> Result<PathBuf, std::io::Error> {
    let project_dirs = get_project_dirs();
    let path = project_dirs.data_dir();
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }

    return Ok(path.to_path_buf());
}

// #endregion helper
