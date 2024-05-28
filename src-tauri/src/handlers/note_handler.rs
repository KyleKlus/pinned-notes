use hsl::HSL;
use rand::{seq::index, Rng};
use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, Manager, PhysicalPosition, Position, Size, Window};
use uuid::Uuid;

use crate::handlers::file_handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub uuid: String,
    pub color: String,
    pub text: String,
    pub pinned: bool,
    pub position: (i32, i32),
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
        position: (note_position.x, note_position.y),
    };

    file_handler::save_note(&new_note).unwrap();

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
            x: note.position.0,
            y: note.position.1,
        }))
        .unwrap();

    note_window.show().unwrap();
}

pub fn delete_all_notes(app: &tauri::AppHandle) {
    let uuids = file_handler::load_all_note_uuids().unwrap();

    // close all windows
    let windows = app.windows();
    for window in windows {
        let window_uuid = window.1.label();
        if !uuids.iter().position(|r| r == window_uuid).is_none() {
            window.1.close().unwrap();
        }
    }

    // delete all notes
    for uuid in uuids {
        file_handler::delete_note(uuid).unwrap();
    }
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
        s: 40.0,
        l: 90.0,
    };
    let random_color = random_hsl_color.to_rgb();

    return format!(
        "#{:02X}{:02X}{:02X}",
        random_color.0, random_color.1, random_color.2
    );
}

// #endregion helper
