use std::{sync::{Mutex, Arc}, thread, time::Duration};

use serde::{self, Serialize};
use tauri;


#[derive(Debug, Serialize)]
enum ModeTimer {
    Focused,
    ShortBreak,
    LongBreak
}


#[derive(Debug, serde::Serialize)]
struct ModelTimer {
    is_running: bool,
    mode: ModeTimer,
    minutes: i32,
    seconds: i32
}


impl Default for ModelTimer {
    fn default() -> Self {
        Self {
            is_running: false,
            mode: ModeTimer::Focused,            
            minutes: 30,
            seconds: 0
        }
    }
}

impl ModelTimer {
    fn reset_time(&mut self, conf: &TomatoConfig) {
        self.set_is_running(false);
        self.minutes = conf.focus_minutes;
        self.seconds = 0;
        self.mode = ModeTimer::Focused;
    }

    fn set_is_running(&mut self, val: bool){
        self.is_running = val;
    }

    fn deincrement(&mut self) {
        if self.minutes == 0 && self.seconds == 0 {
            return 
        }
    }
}

#[derive(Debug, Default)]
struct TomatoConfig {
    focus_minutes: i32,
    relax_minutes: i32,
}


pub struct TomatoTimer {
    config: Mutex<TomatoConfig>,
    model: Arc<Mutex<ModelTimer>>
}

impl TomatoTimer {
    pub fn new() -> Self {
        Self { 
            config: Mutex::new(TomatoConfig::default()), 
            model: Arc::new(Mutex::new(ModelTimer::default())) 
        }
    }
}

#[tauri::command]
pub fn run_timer(timer: tauri::State<TomatoTimer>) {
    let model = Arc::clone(&timer.model);
    thread::spawn(move || {
        model.lock().unwrap().is_running = true;
        loop {
            let dur = Duration::from_secs(1);
            thread::sleep(dur);
            if model.lock().unwrap().is_running == false {
                return 
            }
            model.lock().unwrap().deincrement();
        };
    });
}

#[tauri::command]
pub fn stop_timer(timer: tauri::State<TomatoTimer>) {
    timer.model.lock().unwrap().is_running = false;
}

#[tauri::command]
pub fn reset_timer(timer: tauri::State<TomatoTimer>) {
    let mut model = timer.model.lock().unwrap();
    let config = timer.config.lock().unwrap();
    model.reset_time(&config);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){}
}