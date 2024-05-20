// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rand::random;
use tauri::{
    CustomMenuItem, LogicalSize, Size, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

fn create_system_tray() -> SystemTray {
    let add: CustomMenuItem = CustomMenuItem::new("add".to_string(), "Add note");
    let clear_notes: CustomMenuItem = CustomMenuItem::new("clear_notes".to_string(), "Clear notes");
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(add)
        .add_item(clear_notes)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray: SystemTray = SystemTray::new().with_menu(tray_menu);
    return tray;
}

fn get_new_note_lable() -> String {
    let mut lable: String = "note_".to_owned();
    let random_number: i32 = random();
    lable.push_str(&random_number.to_string());

    return lable;
}

fn create_new_note(app: &tauri::AppHandle) {
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
    .build()
    .unwrap();

    note.set_size(Size::Logical(LogicalSize {
        width: 300.0,
        height: 400.0,
    }))
    .unwrap();
}

#[tauri::command]
async fn create_new_note_from_note(app: tauri::AppHandle) {
    create_new_note(&app);
}

fn main() {
    let app: tauri::App = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_new_note_from_note])
        .system_tray(create_system_tray())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "add" => {
                    // TODO: implement window creation
                    println!("Add note");
                    create_new_note(&app);
                }
                "clear_notes" => {
                    // TODO: Implement window deletion
                    println!("Clear notes");
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, _| {});
}
