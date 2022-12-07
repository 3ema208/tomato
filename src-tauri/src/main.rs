#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


use tauri::{SystemTrayMenu, SystemTray, SystemTrayEvent, CustomMenuItem, Icon, Manager, GlobalShortcutManager};


fn main() {
    let menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("kek".to_owned(), "hahe".to_owned()));
    
    let system_tray = SystemTray::new().with_menu(menu);

    tauri::Builder::default()
        .setup(
            |app| {
                let mut gl_shortcuts = app.global_shortcut_manager();
                gl_shortcuts.register("SHIFT+T", ||{ println!("shift+t") });
                gl_shortcuts.register("SHIFT+P", ||{println!("shift+p")});
                gl_shortcuts.register("SHIFT+C", ||{println!("shift+c")});
                Ok(())
            }
        )
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::LeftClick {tray_id, ..} => {
                    println!("left click {:?}", tray_id);
                },
                SystemTrayEvent::RightClick {tray_id, position, ..} => {
                    println!("right click {:?} {:?}", tray_id, position)
                },
                SystemTrayEvent::MenuItemClick {tray_id, id, ..} => {
                    println!("menu item click {:?} {:?}", tray_id, id)
                }
                SystemTrayEvent::DoubleClick {tray_id, ..} => {
                    println!("double click {:?}", tray_id)
                }
                _ => {println!("default case")}
            }
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
