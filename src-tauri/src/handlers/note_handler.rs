use serde::{Deserialize, Serialize};
use tauri::{LogicalSize, PhysicalPosition, Position, Size, Window};
use uuid::Uuid;

use crate::handlers::file_handler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub uuid: String,
    pub text: String,
    pub pinned: bool,
    pub position: (i32, i32),
}

pub fn reopen_all_notes(app: &tauri::AppHandle) {
    let notes = file_handler::load_notes().unwrap();

    for note in notes {
        reopen_note(app, &note);
    }
}

pub fn reopen_note(app: &tauri::AppHandle, note: &Note) {
    let note_window = open_note_window(app, &note.uuid);
    note_window.set_position(Position::Physical(PhysicalPosition {
        x: note.position.0,
        y: note.position.1,
    })).unwrap();

    note_window.show().unwrap();
}

pub fn create_new_note(app: &tauri::AppHandle) {
    let new_uuid = Uuid::new_v4().to_string();

    let note_window = open_note_window(app, &new_uuid);

    let note_position = note_window.outer_position().unwrap();

    let new_note = Note {
        uuid: new_uuid,
        text: "".to_string(),
        pinned: false,
        position: (note_position.x, note_position.y),
    };

    file_handler::save_note(&new_note).unwrap();

    note_window.show().unwrap();
}

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
