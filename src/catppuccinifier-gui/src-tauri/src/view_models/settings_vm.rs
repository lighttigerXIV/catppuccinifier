use std::fs;

use log::{info, warn};
use serde::{Deserialize, Serialize};
use tauri::Window;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub theme: Theme,
    pub accent: Accent,
    pub show_titlebar: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Theme {
    Mocha,
    Macchiato,
    Frappe,
    Latte,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Accent {
    Rosewater,
    Flamingo,
    Pink,
    Mauve,
    Red,
    Maroon,
    Peach,
    Yellow,
    Green,
    Teal,
    Sky,
    Sapphire,
    Blue,
    Lavender,
}

fn get_default_settings() -> Settings {
    Settings {
        theme: Theme::Mocha,
        accent: Accent::Yellow,
        show_titlebar: true,
    }
}

#[tauri::command]
pub fn vm_set_show_titlebar(window: Window) -> Result<(), String> {
    let show_decorations = vm_get_settings().map_err(|e| e.to_string())?.show_titlebar;

    window
        .set_decorations(show_decorations)
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn vm_get_settings() -> Result<Settings, String> {
    let path = dirs::config_local_dir()
        .ok_or_else(|| "Couldn't get config dir".to_string())?
        .join("catppuccinifier.toml");

    if !path.exists() {
        let settings = get_default_settings();
        let bytes = toml::to_string_pretty(&settings).map_err(|e| e.to_string())?;

        fs::write(&path, &bytes).map_err(|e| e.to_string())?;

        info!("Using default settings");

        return Ok(settings);
    }

    let bytes = fs::read(&path).map_err(|e| e.to_string())?;

    let settings = match toml::from_slice(&bytes) {
        Ok(settings) => settings,
        Err(_) => {
            warn!("Using default settings");
            get_default_settings()
        }
    };

    Ok(settings)
}

#[tauri::command]
pub fn vm_save_settings(settings: Settings) -> Result<(), String> {
    let toml = toml::to_string_pretty(&settings).map_err(|e| e.to_string())?;

    let path = dirs::config_local_dir()
        .ok_or_else(|| "Couldn't get config dir".to_string())?
        .join("catppuccinifier.toml");

    fs::write(&path, &toml).map_err(|e| e.to_string())?;

    Ok(())
}
