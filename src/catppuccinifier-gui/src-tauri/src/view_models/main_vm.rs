use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use catppuccinifier_rs::{
    catppuccinify,
    generation::{Algorithm, Flavor},
};
use rand::{distr::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub struct Properties {
    pub file: PathBuf,
    pub flavor: Flavor,
    pub hald_level: u8,
    pub luminosity: f64,
    pub algorithm: Algorithm,
    pub shape: f64,
    pub nearest: Option<usize>,
    pub mean: f64,
    pub std: f64,
    pub iterations: usize,
    pub power: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneratedImage {
    pub original: PathBuf,
    pub outputs: Vec<PathBuf>,
}

fn get_outputs_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = dirs::cache_dir()
        .ok_or_else(|| "".to_string())?
        .join("catppuccinifier-outputs");

    Ok(path)
}

fn get_tokenized_path(outputs_dir: &Path, path: &Path) -> Result<PathBuf, Box<dyn Error>> {
    let token: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let file_extension = path
        .extension()
        .ok_or_else(|| "Error getting file extension")?
        .to_os_string()
        .into_string()
        .map_err(|_| "Error converting to string".to_string())?;

    Ok(outputs_dir
        .to_owned()
        .join(format!("{token}.{file_extension}")))
}

#[tauri::command]
pub fn vm_clear_outputs() -> Result<(), String> {
    let outputs_dir = get_outputs_dir().map_err(|e| e.to_string())?;

    if outputs_dir.exists() {
        fs::remove_dir_all(&outputs_dir).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn vm_generate_image(properties: Properties) -> Result<PathBuf, String> {
    let outputs_dir = get_outputs_dir().map_err(|e| e.to_string())?;

    if !outputs_dir.exists() {
        fs::create_dir_all(&outputs_dir).map_err(|e| e.to_string())?;
    }

    let output_path =
        get_tokenized_path(&outputs_dir, &properties.file).map_err(|e| e.to_string())?;

    catppuccinify(
        &catppuccinifier_rs::generation::Properties {
            hald_level: properties.clone().hald_level,
            luminosity: properties.clone().luminosity,
            algorithm: properties.clone().algorithm,
            shape: properties.clone().shape,
            nearest: match properties.algorithm {
                Algorithm::LinearRBF => 5,
                _ => 26,
            },
            mean: properties.clone().mean,
            std: properties.clone().std,
            iterations: properties.clone().iterations,
            power: properties.clone().power,
        },
        &properties.flavor,
        &properties.file,
        &output_path,
    )
    .map_err(|e| e.to_string())?;

    Ok(output_path)
}

#[tauri::command(rename_all = "snake_case")]
pub fn vm_save_image(save_path: PathBuf, output: PathBuf) -> Result<(), String> {
    let bytes = fs::read(&output).map_err(|e| e.to_string())?;
    fs::write(&save_path, &bytes).map_err(|e| e.to_string())?;
    Ok(())
}
