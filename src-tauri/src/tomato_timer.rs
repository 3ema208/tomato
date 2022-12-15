
use std::{thread::{self, sleep}, sync::{Arc, Mutex, self, mpsc::{Sender, Receiver, self}}, fmt::Display, time::Duration};

use serde::{Serialize, Deserialize};
use tauri::async_runtime::channel;



#[derive(Debug, Serialize, Deserialize)]
struct ConfigTimer {
    focused_minutes: i32,
    break_minutes: i32,
    amount_round: i8,
}

impl Default for ConfigTimer {
    fn default() -> Self {
        Self {
            focused_minutes: 30,
            break_minutes: 5,
            amount_round: 4 
        }
    }
}

impl ConfigTimer {
    fn set_focused_minutes(&mut self, val: i32) {
        self.focused_minutes = val
    }

    fn set_work_minutes(&mut self, val: i32) {
        self.break_minutes = val
    }

    fn set_amount_rounds(&mut self, val: i8) {
        self.amount_round = val
    }

}


#[derive(Debug, Serialize)]
enum StatesTimer {
    Focused,
    Break,
    Pause, 
    Stopped
}

#[derive(Debug, Serialize)]
struct TimeModel {
    minutes: i32,
    seconds: i32,
}

impl TimeModel {
    fn is_the_end(&self) -> bool {
        self.minutes == 0 && self.seconds == 0
    }

    fn minus_second(&mut self) {
        if self.seconds == 0 && self.minutes != 0 {
            self.minutes -= 1;
            self.seconds = 59;
        } else {
            self.seconds -= 1;
        }
    }

}

impl Display for TimeModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.minutes, self.seconds)
    }
}


#[derive(Debug, Serialize)]
struct TimerModel {
    state: StatesTimer,
    is_running: bool,
    least_time: TimeModel
}

impl Default for TimerModel {
    fn default() -> Self {
        Self {
            state: StatesTimer::Stopped,
            is_running: false,
            least_time: TimeModel { minutes: 0, seconds: 1 }
        }
    }
}

#[derive(Debug)]
struct TomatoTimer {
    config: ConfigTimer,
    model: Arc<Mutex<TimerModel>>,
}

impl Default for TomatoTimer {
    fn default() -> Self {
        Self {
            config: ConfigTimer::default(),
            model: Arc::new(Mutex::new(TimerModel::default())),
        }
    }
}


impl TomatoTimer {
    fn focused(&mut self) {}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works(){
    }

}