// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use catppuccinifier_rs::generate::GenerateProperties;
use settings::Settings;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};
use structs::GenerationData;
use tauri::{AppHandle, Manager};

pub mod settings;
pub mod structs;
use crate::settings::{get_setting, update_setting};

#[tauri::command()]
async fn generate_image(data: GenerationData) -> Result<String, String> {

    let image_name = match Path::new(&data.image_path).file_name() {
        Some(name) => name.to_str().unwrap(),
        None => return Err("Error getting image name".to_owned()),
    };


    let mut save_path = get_temp_dir();
    save_path.push(format!(
        "{}-hald{}-{}",
        data.flavor, data.hald, image_name
    ));

    if !get_temp_dir().exists() {
        let create_dir = fs::create_dir_all(&get_temp_dir());
        if create_dir.is_err() {
            return Err("Error creating temp folder".to_owned());
        }
    }

    let algorithm = match data.algorithm.as_str() {
        "gaussian_rbf" => catppuccinifier_rs::generate::Algorithm::GaussianRBF,
        "gaussian_sampling" => catppuccinifier_rs::generate::Algorithm::GaussianSampling,
        "linear_rbf" => catppuccinifier_rs::generate::Algorithm::LinearRBF,
        "nearest_neighbour" => catppuccinifier_rs::generate::Algorithm::NearestNeighbor,
        _ => catppuccinifier_rs::generate::Algorithm::ShepardsMethod,
    };

    let flavor = match data.flavor.as_str() {
        "latte" => catppuccinifier_rs::generate::Flavor::Latte,
        "frappe" => catppuccinifier_rs::generate::Flavor::Frappe,
        "macchiato" => catppuccinifier_rs::generate::Flavor::Macchiato,
        "mocha" => catppuccinifier_rs::generate::Flavor::Mocha,
        _ => catppuccinifier_rs::generate::Flavor::Oled,
    };

    return match catppuccinifier_rs::generate::generate_image(
        GenerateProperties {
            hald_level: data.hald,
            luminosity: data.luminosity,
            algorithm,
            shape: data.shape,
            nearest: data.nearest,
            mean: data.mean,
            std: data.std,
            iterations: data.iterations,
            power: data.power,
        },
        flavor,
        Path::new(&data.image_path).to_path_buf(),
        Path::new(&save_path.to_owned()).to_path_buf(),
    ) {
        Ok(()) => Ok(save_path.into_os_string().into_string().unwrap()),
        Err(_) => Err("Error generating image".to_owned()),
    };
}

#[tauri::command]
async fn clear_temp_folder() {
    if get_temp_dir().exists() {
        fs::remove_dir_all(get_temp_dir()).expect("Error deleting temp folder");
    }
}

fn get_temp_dir() -> PathBuf {
    return match env::consts::OS {
        "windows" => Path::new("C:\\Windows\\TEMP\\catppuccinifier").to_owned(),
        _ => Path::new("/tmp/catppuccinifier/").to_owned(),
    };
}

#[tauri::command(rename_all = "snake_case")]
async fn save_image(image_path: String, save_path: String) -> Result<String, String> {
    return match fs::copy(&image_path, &save_path) {
        Ok(_) => Ok("Image saved successfully".into()),
        Err(_) => Err("Error saving image".into()),
    };
}

fn init_settings(app_handle: AppHandle) -> Result<(), ()> {
    let mut settings_path = app_handle.path_resolver().app_config_dir().unwrap();
    settings_path.push("settings.json");

    //Creates the folder if it doesn't exist
    if !settings_path.parent().unwrap().exists() {
        fs::create_dir_all(settings_path.parent().unwrap().to_owned())
            .expect("Error creating config folder");
    }

    //Creates the settings file if it doesn't exist
    if !settings_path.exists() {
        let mut file = match File::create(settings_path) {
            Ok(file) => file,
            Err(_) => panic!("Error creating settings file."),
        };

        let settings = Settings {
            theme: "mocha".to_owned(),
            accent: "blue".to_owned(),
        };

        let settings_json = match serde_json::to_string_pretty(&settings) {
            Ok(json) => json,
            Err(_) => panic!("Error obtaining json from settings struct."),
        };

        match file.write_all(settings_json.as_bytes()) {
            Ok(()) => return Ok(()),
            Err(_) => panic!("Error writing settings file."),
        }
    }

    return Ok(());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_image,
            clear_temp_folder,
            save_image,
            get_setting,
            update_setting
        ])
        .setup(|app| {
            init_settings(app.app_handle().to_owned()).unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
