// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn main() {
    let add: CustomMenuItem = CustomMenuItem::new("add".to_string(), "Add note");
    let clear_notes: CustomMenuItem = CustomMenuItem::new("clear_notes".to_string(), "Clear notes");
    let quit: CustomMenuItem = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu: SystemTrayMenu = SystemTrayMenu::new()
        .add_item(add)
        .add_item(clear_notes)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray: SystemTray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        // .invoke_handler(tauri::generate_handler![greet])
        .system_tray(tray)
        .on_system_tray_event(|_app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "add" => {
                    // TODO implement window creation
                    println!("Add note");
                }
                "clear_notes" => {
                    println!("Clear notes");
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
