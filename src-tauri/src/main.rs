#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Mutex, Arc};

use tauri;

mod app_system_tray;
use app_system_tray::{make_system_tray, handler_system_tray_events};

mod tomato;
use tomato::model::TimerModel;
use tomato::{run_timer, get_tomato_period, stop_timer, get_qty_tomato};


fn main() {
    tauri::Builder::default()
        .system_tray(make_system_tray())
        .on_system_tray_event(handler_system_tray_events)
        .invoke_handler(tauri::generate_handler![run_timer, get_tomato_period, stop_timer])
        .manage(Arc::new(Mutex::new(TimerModel::default())))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
