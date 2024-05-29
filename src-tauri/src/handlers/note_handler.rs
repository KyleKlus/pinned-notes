use hsl::HSL;
use rand::Rng;
use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, PhysicalPosition, Position, Size, Window};
use thiserror::Error;
use uuid::Uuid;

use crate::handlers::file_handler;

use super::file_handler::{LoadNoteError, LoadUUIDSError, SaveNoteError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub uuid: String,
    pub color: String,
    pub text: String,
    pub pinned: bool,
    pub x: i32,
    pub y: i32,
}

// #region public

pub fn create_new_note(app: &tauri::AppHandle) {
    let new_uuid = Uuid::new_v4().to_string();

    let note_window = open_note_window(app, &new_uuid);

    let note_position = note_window.outer_position().unwrap();

    let new_note = Note {
        uuid: new_uuid,
        color: get_random_color(),
        text: "".to_string(),
        pinned: false,
        x: note_position.x,
        y: note_position.y,
    };

    file_handler::save_note_to_file(&new_note).unwrap();

    note_window.show().unwrap();
}

pub fn reopen_all_notes(app: &tauri::AppHandle) {
    let notes = file_handler::load_notes().unwrap();

    for note in notes {
        reopen_note(app, &note);
    }
}

pub fn reopen_note(app: &tauri::AppHandle, note: &Note) {
    let note_window = open_note_window(app, &note.uuid);

    note_window
        .set_position(Position::Physical(PhysicalPosition {
            x: note.x,
            y: note.y,
        }))
        .unwrap();

    note_window.show().unwrap();
}

pub fn load_note(uuid: &String) -> Result<Note, LoadNoteError> {
    return file_handler::load_note(uuid, None);
}

pub fn update_note(note: &Note) -> Result<(), SaveNoteError> {
    return file_handler::save_note_to_file(note);
}

#[derive(Error, Debug)]
pub enum DeleteAllNotesError {
    #[error(transparent)]
    LoadUUIDSError(#[from] LoadUUIDSError),
    #[error(transparent)]
    DeleteNoteError(#[from] DeleteNoteError),
}

pub fn delete_all_notes(app: &tauri::AppHandle) -> Result<(), DeleteAllNotesError> {
    let uuids = file_handler::load_all_note_uuids()?;

    // delete all notes
    for uuid in uuids {
        delete_note(app, &uuid)?;
    }

    return Ok(());
}

#[derive(Error, Debug)]
pub enum DeleteNoteError {
    #[error("Delete note file didn't work")]
    IO(#[from] std::io::Error),
    #[error("Note not found with uuid:{}", uuid)]
    NoteNotFound { uuid: String },
}

pub fn delete_note(app: &tauri::AppHandle, uuid: &String) -> Result<(), DeleteNoteError> {
    use DeleteNoteError::*;
    // close all windows
    let app_windows = app.windows();
    let uuid_window = app_windows.get_key_value(uuid);
    match uuid_window {
        Some(window) => window.1.close().unwrap(),
        None => {
            return Err(NoteNotFound {
                uuid: uuid.to_string(),
            })
        }
    }

    file_handler::delete_note_file(uuid)?;
    return Ok(());
}

// #endregion public

// #region helper

fn open_note_window(app: &tauri::AppHandle, uuid: &String) -> Window {
    let note_window = tauri::WindowBuilder::new(
        app,
        uuid.clone(), /* the unique window label */
        tauri::WindowUrl::App("index.html".parse().unwrap()),
    )
    .decorations(false)
    .center()
    .transparent(true)
    .skip_taskbar(true)
    .title(uuid.clone())
    .maximizable(false)
    .minimizable(false)
    .visible(false)
    .build()
    .unwrap();

    note_window
        .set_size(Size::Logical(LogicalSize {
            width: 300.0,
            height: 400.0,
        }))
        .unwrap();

    return note_window;
}

fn get_random_color() -> String {
    let mut rng = rand::thread_rng();
    let random_hsl_color = HSL {
        h: rng.gen_range(0.0..360.0),
        s: 0.4,
        l: 0.9,
    };
    let random_color = random_hsl_color.to_rgb();

    return format!(
        "#{:02X}{:02X}{:02X}",
        random_color.0, random_color.1, random_color.2
    );
}

// #endregion helper
