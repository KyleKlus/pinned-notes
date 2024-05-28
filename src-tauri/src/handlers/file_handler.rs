use crate::handlers::note_handler::Note;
use directories::ProjectDirs;

use serde_json;
use std::{fmt, fs};
use tauri::api::file;

#[derive(Debug)]
pub struct NoteCreationError {
    reason: String,
}

#[derive(Debug)]
pub struct NoteReadError {
    reason: String,
}

impl fmt::Display for NoteCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The creation of a note wasn't possible.\nReason: {}",
            self.reason
        )
    }
}

pub fn load_notes() -> Result<Vec<Note>, NoteReadError> {
    // Check if the app directory exists
    let proj_dirs = match ProjectDirs::from("com", "Kyle Klus", "Notes") {
        Some(dirs) => dirs,
        None => {
            return Err(NoteReadError {
                reason: "Couldn't find project directory".to_string(),
            });
        }
    };

    // Create the app directory if it doesn't exist
    let path = proj_dirs.data_dir();
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(_) => return Ok(Vec::new()),
            Err(e) => {
                return Err(NoteReadError {
                        reason: format!("Couldn't create app directory after checking, that a new one is needed: {}", e),
                    });
            }
        }
    }
    // TODO: clean this mess
    let notes_it = std::fs::read_dir(path)
        .unwrap()
        .map(|f| {
            return f.unwrap().path();
        })
        .map(|fp| {
            return std::fs::read_to_string(fp).unwrap();
        })
        .map(|fc| {
            let note: Note = serde_json::from_str(&fc).unwrap();
            return note;
        });

    let mut notes = Vec::new();

    for note in notes_it {
        notes.push(note);
    }

    return Ok(notes);
}

pub fn save_note(note: &Note) -> Result<(), NoteCreationError> {
    // Check if the app directory exists
    let proj_dirs = match ProjectDirs::from("com", "Kyle Klus", "Notes") {
        Some(dirs) => dirs,
        None => {
            return Err(NoteCreationError {
                reason: "Couldn't find project directory".to_string(),
            });
        }
    };

    // Create the app directory if it doesn't exist
    let path = proj_dirs.data_dir();
    if !path.exists() {
        match std::fs::create_dir_all(path) {
            Ok(_) => (),
            Err(e) => {
                return Err(NoteCreationError {
                        reason: format!("Couldn't create app directory after checking, that a new one is needed: {}", e),
                    });
            }
        }
    }

    // Serialize the note to json
    let json_data = match serde_json::to_string(&note) {
        Ok(data) => data,
        Err(e) => {
            return Err(NoteCreationError {
                reason: format!("Couldn't parse object to json: {}", e),
            });
        }
    };

    // Write the json data to a file
    match fs::write(path.join(format!("{}.txt", &note.uuid)), json_data) {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(NoteCreationError {
                reason: format!(
                    "Couldn't create app directory after checking, that a new one is needed: {}",
                    e
                ),
            });
        }
    }
}
