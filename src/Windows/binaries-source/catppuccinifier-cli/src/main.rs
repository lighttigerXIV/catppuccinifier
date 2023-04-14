use std::env;
use std::process::Command;
use substring::Substring;
use clap::{command, ArgAction, Arg};

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
                .default_value("all")
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
                .default_value("4")
        )
        .arg(
            Arg::new("image")
                .short('i')
                .long("image")
                .action(ArgAction::Set)
                .value_name("IMAGE")
                .help("Selects the image to apply the flavor")
                .required(true)
        )
}

fn convert_image(
    flavor: &str,
    noise_level: &str,
    image: &str,
    current_folder_path: &str,
    catppuccinifier_folder_path: &str,
) {

    println!("Generating {} image with noise level {}", flavor, noise_level);

    let command = format!(
        r"magick convert '{}\{}' '{}\flavors\noise-{}\{}.png' -hald-clut '{}\{}-noise{}-{}'",
        current_folder_path,
        image,
        catppuccinifier_folder_path,
        noise_level,
        flavor,
        current_folder_path,
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
        println!("An error occurred: {}", String::from_utf8_lossy(&flavor_command.stderr));
    }
}

fn main() {
    let matches = get_cli().get_matches();
    let flavors_reference = matches.get_many::<String>("flavor");
    let noise_level = matches.get_one::<String>("noise");
    let image_path = matches.get_one::<String>("image");

    match env::current_dir() {
        Ok(current_folder) => {
            let current_folder_path = current_folder.to_str().unwrap();
            let catppuccinifier_folder_path = r"C:\Program Files\Catppuccinifier";

            let mut image= "";

            if image_path.unwrap().contains(r".\") {
                image = image_path.unwrap().substring(2, image_path.unwrap().len());
            } else {
                image = image_path.unwrap()
            }

            match flavors_reference {
                Some(flavors) => {
                    for flavor in flavors {
                        if flavor != "all" {
                            convert_image(
                                flavor,
                                noise_level.unwrap(),
                                image,
                                current_folder_path,
                                catppuccinifier_folder_path,
                            );
                        }

                        if flavor == "all"{

                            let possible_flavors = ["latte", "frappe", "macchiato", "mocha", "oled"];

                            for possible_flavor in possible_flavors{

                                convert_image(
                                    possible_flavor,
                                    noise_level.unwrap(),
                                    image,
                                    current_folder_path,
                                    catppuccinifier_folder_path,
                                );
                            }
                        }
                    }
                }
                None => {}
            }
        }
        Err(_) => {
            println!("ERROR: Couldn't get current folder");
        }
    }
}
