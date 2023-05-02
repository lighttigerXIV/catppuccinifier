// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::{env, fs};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;




#[tauri::command(rename_all = "snake_case")]
async fn generate_image(
    image_path: String,
    noise_level: String,
    flavor: String,
) -> Result<String, String> {
    let image_extension = Path::new(&image_path).extension().unwrap().to_str().unwrap();
    let random_name: String = thread_rng().sample_iter(&Alphanumeric).take(14).map(char::from).collect();


    match env::consts::OS {
        "linux" => {
            if !Path::new("/tmp/catppuccinifier").exists() {
                fs::create_dir("/tmp/catppuccinifier")
                    .expect("");
            }

            return match Path::new("/tmp/catppuccinifier").exists() {
                true => {
                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/{}.png -hald-clut /tmp/catppuccinifier/{}.{}", image_path, noise_level, flavor, random_name, &image_extension);

                    let result = Command::new("/bin/sh")
                        .arg("-c")
                        .arg(&command)
                        .output()
                        .expect("");

                    if result.status.success() {
                        Ok(format!("/tmp/catppuccinifier/{}.{}", random_name, image_extension))
                    } else {
                        Err("Error converting image".into())
                    }
                }
                false => {
                    Err("Error converting image".into())
                }
            }
        },
        "windows"=>{
            return match env::var("TEMP") {
                Ok(temp_path) => {
                    let temp_catppuccinifier_path = format!("{}\\catppuccinifier", &temp_path);

                    if !Path::new(&temp_catppuccinifier_path).exists() {
                        fs::create_dir(&temp_catppuccinifier_path)
                            .expect("");
                    }

                    match Path::new(&temp_catppuccinifier_path).exists() {
                        true => {
                            let command = format!("magick convert '{}' 'C:\\Program Files\\Catppuccinifier\\flavors\\noise-{}\\{}.png' -hald-clut '{}\\{}.{}'", image_path, noise_level, flavor, &temp_catppuccinifier_path, &random_name, &image_extension);

                            let result = Command::new("powershell")
                                .arg("-Command")
                                .arg(&command)
                                .creation_flags(0x08000000)
                                .output()
                                .expect("");

                            return if result.status.success() {
                                Ok(format!("{}\\{}.{}", temp_catppuccinifier_path, random_name, image_extension))
                            } else {
                                Err("Error converting image".into())
                            }
                        }
                        false => {
                            Err("Error converting image".into())
                        }
                    }
                },
                Err(_) => {
                    Err("Error getting temp variable".into())
                }
            }
        }
        _ => { Err("OS not supported".into()) }
    }
}

#[tauri::command]
async fn clear_temp_folder(){

    match env::consts::OS {
        "linux"=>{
            if Path::new("/tmp/catppuccinifier/").exists(){
                fs::remove_dir_all("/tmp/catppuccinifier").expect("Error deleting temp folder");
            }
        },
        "windows"=>{
            if Path::new("C:\\Windows\\TEMP\\catppuccinifier").exists(){
                fs::remove_dir_all("C:\\Windows\\TEMP\\catppuccinifier").expect("Error deleting temp folder");
            }
        }
        _ =>{}
    }
}

#[tauri::command(rename_all = "snake_case")]
async fn save_image(
    image_path: String,
    saved_path: String
)-> Result<String, String>{

    return match fs::copy(&image_path, &saved_path) {
        Ok(_) => {
            Ok("Image saved successfully".into())
        },
        Err(_) => {
            Err("Error saving image".into())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate_image, clear_temp_folder, save_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
