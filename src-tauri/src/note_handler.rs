use rand::random;
use tauri::{LogicalSize, Size};

pub fn create_new_note(app: &tauri::AppHandle) {
    initialize_new_note_data();

    let note = tauri::WindowBuilder::new(
        app,
        get_new_note_lable(), /* the unique window label */
        tauri::WindowUrl::App("index.html".parse().unwrap()),
    )
    .decorations(false)
    .center()
    .transparent(true)
    .skip_taskbar(true)
    .title("Note")
    .maximizable(false)
    .minimizable(false)
    .build()
    .unwrap();

    note.set_size(Size::Logical(LogicalSize {
        width: 300.0,
        height: 400.0,
    }))
    .unwrap();
}

fn get_new_note_lable() -> String {
    let mut lable: String = "note_".to_owned();
    let random_number: i32 = random();
    lable.push_str(&random_number.to_string());

    return lable;
}

fn initialize_new_note_data() {}
