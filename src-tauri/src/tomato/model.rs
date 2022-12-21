use std::time::{Duration};

use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum TimerMode {
    Focusing,
    ShortBreak,
    LongBreak,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct TimerModel {
    pub is_running: bool,
    pub delay: Duration,
    pub qty_tomato: i32,
    pub mode: TimerMode,
}

impl Default for TimerModel {
    fn default() -> Self {
        Self {
            is_running: false,
            delay: Duration::from_secs(60 *30),
            qty_tomato: 0,
            mode: TimerMode::Focusing
        }
    }
}


impl TimerModel {
    pub fn new() -> Self {
        Self {
            is_running: false,
            delay: Duration::from_secs(60 * 30),
            qty_tomato: 0,
            mode: TimerMode::Focusing
        }
    }

    pub fn minus_time(&mut self, time: Duration) {
        self.delay -= time;
    }

    pub fn set_running(&mut self, state: bool) {
        self.is_running = state;
    }

    pub fn set_mode(&mut self, mode: TimerMode) {
        self.mode = mode;
    }
}