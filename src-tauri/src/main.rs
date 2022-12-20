#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri;

mod app_system_tray;
mod tomato_timer;
use app_system_tray::{make_system_tray, handler_system_tray_events};
use tomato_timer::{run_timer, stop_timer, TomatoTimer};


fn main() {
    tauri::Builder::default()
        .system_tray(make_system_tray())
        .on_system_tray_event(handler_system_tray_events)
        .manage(TomatoTimer::new())
        .invoke_handler(tauri::generate_handler![run_timer, stop_timer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
