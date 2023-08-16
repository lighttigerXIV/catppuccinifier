use std::{
    fs::{self},
    path::PathBuf,
};

use tauri::AppHandle;

//Paths
pub fn get_settings_path(app_handle: AppHandle) -> Result<PathBuf, ()> {
    let mut settings_path = app_handle.path_resolver().app_config_dir().ok_or(())?;
    settings_path.push("settings.json");
    return Ok(settings_path);
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Settings {
    pub theme: String,
    pub accent: String,
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_setting(setting: String, default: String, app_handle: AppHandle) -> String {
    let settings_path = match get_settings_path(app_handle) {
        Ok(path) => path,
        Err(_) => return default,
    };

    let settings_content = match fs::read_to_string(settings_path) {
        Ok(content) => content,
        Err(_) => return default,
    };

    let settings: Settings = match serde_json::from_str(&settings_content) {
        Ok(settings) => settings,
        Err(_) => return default,
    };

    return match setting.as_str() {
        "theme" => settings.theme,
        "accent" => settings.accent,
        _ => default,
    };
}

#[tauri::command(rename_all = "snake_case")]
pub fn update_setting(setting: &str, value: String, app_handle: AppHandle) -> Result<(), String> {
    let settings_path = match get_settings_path(app_handle) {
        Ok(path) => path,
        Err(_) => return Err("Error getting settings path".to_owned()),
    };

    let settings_content = match fs::read_to_string(settings_path.to_owned()) {
        Ok(content) => content,
        Err(_) => return Err("Error getting settings content".to_owned()),
    };

    let mut settings: Settings = match serde_json::from_str(&settings_content) {
        Ok(settings) => settings,
        Err(_) => return Err("Error getting settings".to_owned()),
    };

    match setting {
        "theme" => settings.theme = value,
        "accent" => settings.accent = value,
        _ => {}
    }

    let new_settings_json = match serde_json::to_string_pretty(&settings) {
        Ok(json) => json,
        Err(_) => return Err("Error converting settings json".to_owned()),
    };

    return match fs::write(settings_path.to_owned(), new_settings_json.as_bytes()) {
        Ok(_)=> Ok(()),
        Err(_)=> Err("Error writing settings file".to_owned())
    }
}
