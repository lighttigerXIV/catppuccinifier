use clap::{command, Arg, ArgAction};
use substring::Substring;

use std::env;
use std::path::Path;

fn get_cli() -> clap::Command {
    command!()
        .version("3.0")
        .name("Catppuccinifier")
        .about("A cli tool to catppuccinify your wallpapers")
        .author("lighttigerXIV")
        .arg(
            Arg::new("flavor")
                .short('f')
                .long("flavor")
                .action(ArgAction::Set)
                .value_name("FLAVOR")
                .num_args(1..6)
                .help("Selects the flavor to apply to the image")
                .value_delimiter(' ')
                .value_parser(["latte", "frappe", "macchiato", "mocha", "oled", "all"])
                .default_value("all"),
        )
        .arg(
            Arg::new("noise")
                .short('n')
                .long("noise")
                .action(ArgAction::Set)
                .value_name("NOISE")
                .num_args(1)
                .help("Selects the noise level to apply to the image")
                .value_parser(["0", "1", "2", "3", "4"])
                .default_value("4"),
        )
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .action(ArgAction::Set)
                .value_name("IMAGE")
                .help("Selects the image to apply the flavor")
                .required(true),
        )
}

#[cfg(target_os = "linux")]
fn generate_image_linux(noise_level: &str, image: &str, flavor: &str) -> Result<(), ()> {
    use std::process::Command;

    let exec_dir = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => return Err(()),
    };

    let home_dir = match dirs::home_dir() {
        Some(path) => path.to_str().unwrap().to_string(),
        None => return Err(()),
    };

    let command = format!(
            "convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/{}.png' -hald-clut '{}-noise{}-{}'",
            exec_dir, image, home_dir, noise_level, flavor, flavor, noise_level, image
        );

    let convert_image = Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("");

    return if convert_image.status.success() {
        Ok(())
    } else {
        Err(())
    };
}

#[cfg(target_os = "windows")]
fn generate_image_windows(noise_level: &str, image: &str, flavor: &str) -> Result<(), ()> {
    use std::process::Command;

    let exec_dir = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => return Err(()),
    };

    let command = format!(
        "magick convert '{}\\{}' 'C:\\Program Files\\Catppuccinifier\\flavors\\noise-{}\\{}.png' -hald-clut '{}-noise{}-{}'",
        exec_dir,
        image,
        noise_level,
        flavor,
        flavor,
        noise_level,
        image
    );

    let flavor_command = Command::new("powershell")
        .arg("-Command")
        .arg(&command)
        .output()
        .expect("ERROR: Couldn't convert image");

    if !flavor_command.status.success() {
        println!(
            "An error occurred: {}",
            String::from_utf8_lossy(&flavor_command.stderr)
        );
    }
    return Ok(());
}

fn main() {
    let matches = get_cli().get_matches();
    let flavors_reference = matches.get_many::<String>("flavor");
    let noise_level = matches.get_one::<String>("noise").unwrap();
    let image_path = matches.get_one::<String>("image").unwrap();

    let os = env::consts::OS;
    let image_extension = match Path::new(image_path).extension() {
        Some(extension) => extension.to_str().unwrap().to_string(),
        None => "".to_string(),
    };
    let image_exists = Path::new(image_path).exists();

    #[cfg(target_os = "linux")]
    if os == "linux" {
        if !image_exists {
            println!("{}", "Couldn't find image".red());
            std::process::exit(1)
        }

        if !["jpg", "jpeg", "png", "webp"].contains(&image_extension) {
            println!(
                "{}",
                "Possible image types are [jpg, jpeg, png, webp]".red()
            );
            std::process::exit(1)
        }

        match flavors_reference {
            Some(flavors) => {
                for flavor in flavors {
                    if flavor == "all" {
                        for possible_flavor in ["latte", "frappe", "macchiato", "mocha", "oled"] {
                            match generate_image_linux(noise_level, image, possible_flavor) {
                                Ok(()) => {
                                    use colored::Colorize;

                                    println!(
                                        "{}",
                                        format!("Successfully generated {} image", possible_flavor)
                                            .green()
                                    )
                                }
                                Err(()) => {
                                    use colored::Colorize;

                                    println!(
                                        "{}",
                                        format!("Error generating {} image", possible_flavor).red()
                                    )
                                }
                            }
                        }
                    } else {
                        match generate_image_linux(noise_level, image, &flavor) {
                            Ok(()) => {
                                println!(
                                    "{}",
                                    format!("Successfully generated {} image", flavor).green()
                                )
                            }
                            Err(()) => {
                                println!("{}", format!("Error generating {} image", flavor).red())
                            }
                        }
                    }
                }
            }
            None => {
                println!("{}", "Error getting flavors".red())
            }
        }
    }

    #[cfg(target_os = "windows")]
    if os == "windows" {
        if !image_exists {
            println!("Couldn't find image");
            std::process::exit(1)
        }

        if !["jpg", "jpeg", "png", "webp"].contains(&image_extension.as_str()) {
            println!("Possible image types are [jpg, jpeg, png, webp]");
            std::process::exit(1)
        }

        match flavors_reference {
            Some(flavors) => {
                for flavor in flavors {

                    let image = if image_path.contains(r".\") {
                        image_path.substring(2, image_path.len())
                    } else {
                        image_path
                    };

                    if flavor == "all" {
                        for possible_flavor in ["latte", "frappe", "macchiato", "mocha", "oled"] {
                            match generate_image_windows(noise_level, image, possible_flavor) {
                                Ok(()) => {
                                    println!("Successfully generated {} image", possible_flavor)
                                }
                                Err(()) => {
                                    println!("Error generating {} image", possible_flavor)
                                }
                            }
                        }
                    } else {
                        match generate_image_windows(noise_level, image, &flavor) {
                            Ok(()) => {
                                println!("Successfully generated {} image", flavor)
                            }
                            Err(()) => {
                                println!("Error generating {} image", flavor)
                            }
                        }
                    }
                }
            }
            None => {}
        }
    }

    if !["linux", "windows"].contains(&os) {
        println!("OS not supported ;-; | Catppuccinfier only runs in Linux and Windows");
    }
}
