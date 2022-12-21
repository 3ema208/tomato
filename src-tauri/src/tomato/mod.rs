use std::{thread, sync::{Mutex, Arc}, time::Duration, fmt::format};

use tauri::Manager;

pub mod model;
use model::TimerModel;


type type_timer = Arc<Mutex<TimerModel>>;


#[tauri::command]
pub fn get_qty_tomato(model: tauri::State<type_timer>) -> i32 {
    model.lock().unwrap().qty_tomato
}


#[tauri::command]
pub fn get_tomato_period(timer_model: tauri::State<type_timer>) -> Result<String, String> {
    match timer_model.lock() {
        Ok(model) => {
            let total_milliseconds = model.delay.as_millis();
            // 1 sec -> 1000 ms
            // 1 min -> 60 sec -> 60000 ms
            let min = total_milliseconds / (60 * 1000);
            let sec = (total_milliseconds - (min * 60 * 1000)) / 1000;
            let mil_sec = total_milliseconds - (min * 60 * 1000) - (sec * 1000); 
            let result = format!("{:02}:{:02}:{:03}", min, sec, mil_sec);
            Ok(result)
        }, 
        _ => Err("99:99".to_string())
    }   
}


#[tauri::command]
pub fn run_timer(app: tauri::AppHandle, timer_model: tauri::State<'_, Arc<Mutex<TimerModel>>>){
    let model = Arc::clone(timer_model.inner());
    thread::spawn(move || {
        match model.lock() {
            Ok(mut model) => {
                if model.is_running == true {
                    // case when thread has been already run before
                    return 
                }
                model.set_running(true);
            }
            _ => {}
        }
        let period = Duration::from_millis(1);
        loop {
            thread::sleep(period);
            match model.lock() {
                Ok(mut model) => {
                    if model.is_running == false {
                        app.emit_all("period_stopped", ()).unwrap();
                        return 
                    }
                    if model.delay.is_zero() {
                        model.set_running(false);
                        app.emit_all("period_finish", ()).unwrap();
                        return 
                        // decide what will do next
                    } else {
                        model.minus_time(period);
                    }
                },
                _ => {}
            }
        }
    });
    stop_timer(timer_model);
}


#[tauri::command]
pub fn stop_timer(timer_model: tauri::State<'_, Arc<Mutex<TimerModel>>>){
    match timer_model.lock() {
        Ok(mut m) => {
            if m.is_running == true {
                m.set_running(false)
            }
        },
        _ => {}
    }
}

#[tauri::command]
pub fn reset_timer(){}
