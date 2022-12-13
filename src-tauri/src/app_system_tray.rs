use std::str::FromStr;

use tauri::{SystemTray, CustomMenuItem, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem, AppHandle};

const RUN_TITPLE: &str = "Run";
const PAUSE_TITLE: &str = "Pause";
const SETTINGS_TITLE: &str = "Settings";

enum AppMenuItem {
    RUN,
    PAUSE,
    SETTINGS
}

impl FromStr for AppMenuItem {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            RUN_TITPLE => Ok(Self::RUN),
            PAUSE_TITLE => Ok(Self::PAUSE),
            SETTINGS_TITLE => Ok(Self::SETTINGS),
            _ => Err("Invalid Command".to_string())
        }
    }
}

impl AppMenuItem {
    fn get_title(&self) -> &'static str {
        match &self {
            Self::RUN => RUN_TITPLE,
            Self::PAUSE => PAUSE_TITLE,
            Self::SETTINGS => SETTINGS_TITLE
        }
    }
}

fn make_custom_menu_item(item: AppMenuItem) -> CustomMenuItem {
    let id_and_title = item.get_title();
    CustomMenuItem::new(id_and_title, id_and_title)
}

pub fn make_system_tray() -> SystemTray {
    SystemTray::new().with_menu(
        SystemTrayMenu::new()
            .add_item(make_custom_menu_item(AppMenuItem::RUN))
            .add_item(make_custom_menu_item(AppMenuItem::PAUSE))
            .add_native_item(SystemTrayMenuItem::Separator)
            .add_item(make_custom_menu_item(AppMenuItem::SETTINGS))
    )
}



pub fn handler_system_tray_events(_app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            handler_event_menu_items(_app, id)
        },
        _ => {println!("default")}
    }
}


fn handler_event_menu_items(_app: &AppHandle, id: String) {
    match AppMenuItem::from_str(id.as_str()) {
        Ok(AppMenuItem::RUN) => {
            println!("RUN command")
        },
        Ok(AppMenuItem::PAUSE) => {
            println!("PAUSE command")
        },
        Ok(AppMenuItem::SETTINGS) => {
            println!("Settings command")
        } 
        Err(e) => {
            println!("default {:?}", e)
        },
    }
}
