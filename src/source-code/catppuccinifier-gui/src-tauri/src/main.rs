// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use exoquant::SimpleColorSpace;
use image::open;
use lutgen::identity::correct_image;
use lutgen::interpolation::{
    GaussianRemapper, GaussianSamplingRemapper, LinearRemapper, NearestNeighborRemapper,
    ShepardRemapper,
};
use lutgen::{GenerateLut, Palette};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::path::Path;
use std::{env, fs};

const SEED: u64 = u64::from_be_bytes(*b"42080085");

#[tauri::command(rename_all = "snake_case")]
async fn generate_image(
    image_path: String,
    hald_level: u8,
    flavor: String,
    conversion_method: String,
    gaussian_euclide: f64,
    gaussian_nearest: usize,
    gaussian_sampling_mean: f64,
    gaussian_sampling_std: f64,
    gaussian_sampling_iterations: usize,
    linear_nearest: usize,
    sheppard_power: f64,
    sheppard_nearest: usize,
) -> Result<String, String> {
    let image_extension = Path::new(&image_path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    let random_name: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(14)
        .map(char::from)
        .collect();

    match env::consts::OS {
        #[cfg(target_os = "linux")]
        "linux" => {
            if !Path::new("/tmp/catppuccinifier").exists() {
                fs::create_dir("/tmp/catppuccinifier").expect("");
            }

            return match Path::new("/tmp/catppuccinifier").exists() {
                true => {
                    match generate_image_in_linux(
                        image_path.to_string(),
                        hald_level,
                        flavor.to_string(),
                        random_name.to_string(),
                        image_extension.to_string(),
                        conversion_method,
                        gaussian_euclide,
                        gaussian_nearest,
                        gaussian_sampling_mean,
                        gaussian_sampling_std,
                        gaussian_sampling_iterations,
                        linear_nearest,
                        sheppard_power,
                        sheppard_nearest,
                    )
                    .await
                    {
                        Ok(image) => return Ok(image),
                        Err(error) => return Err(error.into()),
                    };
                }
                false => Err("Error converting image".into()),
            };
        }
        #[cfg(target_os = "windows")]
        "windows" =>{

            if !Path::new("C:\\Windows\\Temp\\catppuccinifier").exists() {
                fs::create_dir("C:\\Windows\\Temp\\catppuccinifier").expect("");
            }

            return match Path::new("C:\\Windows\\Temp\\catppuccinifier").exists() {
                true => {
                    match generate_image_in_windows(
                        image_path.to_string(),
                        hald_level,
                        flavor.to_string(),
                        random_name.to_string(),
                        image_extension.to_string(),
                        conversion_method,
                        gaussian_euclide,
                        gaussian_nearest,
                        gaussian_sampling_mean,
                        gaussian_sampling_std,
                        gaussian_sampling_iterations,
                        linear_nearest,
                        sheppard_power,
                        sheppard_nearest,
                    )
                    .await
                    {
                        Ok(image) => return Ok(image),
                        Err(error) => return Err(error.into()),
                    };
                }
                false => Err("Error converting image".into()),
            };
        },
        _ => Err("OS not supported".into()),
    }
}

#[cfg(target_os = "linux")]
async fn generate_image_in_linux(
    image_path: String,
    hald_level: u8,
    flavor: String,
    random_name: String,
    image_extension: String,
    conversion_method: String,
    gaussian_euclide: f64,
    gaussian_nearest: usize,
    gaussian_sampling_mean: f64,
    gaussian_sampling_std: f64,
    gaussian_sampling_iterations: usize,
    linear_nearest: usize,
    sheppard_power: f64,
    sheppard_nearest: usize,
) -> Result<String, String> {

    let palette = match flavor.as_str() {
        "mocha" => Palette::CatppuccinMocha.get(),
        "macchiato" => Palette::CatppuccinMacchiato.get(),
        "frappe" => Palette::CatppuccinFrappe.get(),
        "latte" => Palette::CatppuccinLatte.get(),
        _ => Palette::CatppuccinOled.get(),
    };

    let hald_clut = match conversion_method.as_str() {
        "gaussian" => GaussianRemapper::new(
            &palette,
            gaussian_euclide,
            gaussian_nearest,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        "gaussian_sampling" => GaussianSamplingRemapper::new(
            &palette,
            gaussian_sampling_mean,
            gaussian_sampling_std,
            gaussian_sampling_iterations,
            SEED,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        "linear" => LinearRemapper::new(&palette, linear_nearest, SimpleColorSpace::default())
            .generate_lut(hald_level),

        "sheppard" => ShepardRemapper::new(
            &palette,
            sheppard_power,
            sheppard_nearest,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        _ => NearestNeighborRemapper::new(&palette, SimpleColorSpace::default())
            .generate_lut(hald_level),
    };

    let lut_was_generated = match hald_clut.save("/tmp/catppuccinifier/lut.png") {
        Err(_) => false,
        Ok(_) => true,
    };

    if lut_was_generated {
        let mut new_image = open(image_path).unwrap().to_rgb8();
        correct_image(&mut new_image, &hald_clut);

        let save_path = format!("/tmp/catppuccinifier/{}.{}", &random_name, &image_extension);

        match new_image.save(&save_path) {
            Ok(_) => return Ok(save_path.into()),
            Err(_) => return Err("Error converting image".into()),
        };
    } else {
        return Err("".into());
    }
}

#[cfg(target_os = "windows")]
async fn generate_image_in_windows(
    image_path: String,
    hald_level: u8,
    flavor: String,
    random_name: String,
    image_extension: String,
    conversion_method: String,
    gaussian_euclide: f64,
    gaussian_nearest: usize,
    gaussian_sampling_mean: f64,
    gaussian_sampling_std: f64,
    gaussian_sampling_iterations: usize,
    linear_nearest: usize,
    sheppard_power: f64,
    sheppard_nearest: usize,
) -> Result<String, String> {
    let palette = match flavor.as_str() {
        "mocha" => Palette::CatppuccinMocha.get(),
        "macchiato" => Palette::CatppuccinMacchiato.get(),
        "frappe" => Palette::CatppuccinFrappe.get(),
        "latte" => Palette::CatppuccinLatte.get(),
        _ => Palette::CatppuccinOled.get(),
    };

    let hald_clut = match conversion_method.as_str() {
        "gaussian" => GaussianRemapper::new(
            &palette,
            gaussian_euclide,
            gaussian_nearest,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        "gaussian_sampling" => GaussianSamplingRemapper::new(
            &palette,
            gaussian_sampling_mean,
            gaussian_sampling_std,
            gaussian_sampling_iterations,
            SEED,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        "linear" => LinearRemapper::new(&palette, linear_nearest, SimpleColorSpace::default())
            .generate_lut(hald_level),

        "sheppard" => ShepardRemapper::new(
            &palette,
            sheppard_power,
            sheppard_nearest,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        _ => NearestNeighborRemapper::new(&palette, SimpleColorSpace::default())
            .generate_lut(hald_level),
    };

    let lut_was_generated = match hald_clut.save("C:\\Windows\\TEMP\\catppuccinifier\\lut.png") {
        Err(error) => { println!("{}", error); false} ,
        Ok(_) => true,
    };

    if lut_was_generated {
        let mut new_image = open(image_path).unwrap().to_rgb8();
        correct_image(&mut new_image, &hald_clut);

        let save_path = format!("C:\\Windows\\TEMP\\catppuccinifier\\{}.{}", &random_name, &image_extension);

        match new_image.save(&save_path) {
            Ok(_) => return Ok(save_path.into()),
            Err(error) => return Err(error.to_string().into()),
        };
    } else {
        return Err("Lut wasnt generated".into());
    }
}

#[tauri::command]
async fn get_os() -> String {
    let os = env::consts::OS;
    return os.to_string();
}

#[tauri::command]
async fn clear_temp_folder() {
    match env::consts::OS {
        "linux" => {
            if Path::new("/tmp/catppuccinifier/").exists() {
                fs::remove_dir_all("/tmp/catppuccinifier").expect("Error deleting temp folder");
            }
        }
        "windows" => {
            if Path::new("C:\\Windows\\TEMP\\catppuccinifier").exists() {
                fs::remove_dir_all("C:\\Windows\\TEMP\\catppuccinifier")
                    .expect("Error deleting temp folder");
            }
        }
        _ => {}
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn save_image(image_path: String, saved_path: String) -> Result<String, String> {
    return match fs::copy(&image_path, &saved_path) {
        Ok(_) => Ok("Image saved successfully".into()),
        Err(_) => Err("Error saving image".into()),
    };
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            generate_image,
            clear_temp_folder,
            save_image,
            get_os
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
