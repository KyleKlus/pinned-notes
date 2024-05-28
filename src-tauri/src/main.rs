// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
mod handlers;
use crate::handlers::note_handler;

#[tauri::command]
async fn create_new_note_from_note(app: tauri::AppHandle) {
    note_handler::create_new_note(&app);
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
                    note_handler::create_new_note(&app);
                }
                "clear_notes" => {
                    note_handler::delete_all_notes(&app);
                }
                _ => {}
            },
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    note_handler::reopen_all_notes(&app.handle());

    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

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
