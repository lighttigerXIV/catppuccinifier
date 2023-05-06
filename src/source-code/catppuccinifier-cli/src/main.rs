use clap::{command, Arg, ArgAction};
use colored::Colorize;
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

fn main() {
    let matches = get_cli().get_matches();
    let flavors_reference = matches.get_many::<String>("flavor");
    let noise_level = matches.get_one::<String>("noise").unwrap();
    let image = matches.get_one::<String>("image").unwrap();

    let image_extension = Path::new(image).extension().unwrap().to_str().unwrap();
    let image_exists = Path::new(image).exists();

    if !image_exists {
        println!("{}", "Couldn't find image".red());
        std::process::exit(1)
    }

    if !["jpg", "jpeg", "png", "webp"].contains(&image_extension) {
        println!("{}", "Possible image types are [jpg, jpeg, png, webp]".red());
        std::process::exit(1)
    }

    match env::consts::OS {
        "linux" => match flavors_reference {
            Some(flavors) => {
                for flavor in flavors {
                    if flavor == "all" {
                        for possible_flavor in ["latte", "frappe", "macchiato", "mocha", "oled"] {
                            match generate_image_linux(noise_level, image, possible_flavor) {
                                Ok(()) => {
                                    println!(
                                        "{}",
                                        format!("Successfully generated {} image", possible_flavor)
                                            .green()
                                    )
                                }
                                Err(()) => {
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
        },
        _ => {
            println!(
                "{}",
                "OS not supported ;-; | Catppuccinfier only runs in Linux and Windows".red()
            );
        }
    }
}
