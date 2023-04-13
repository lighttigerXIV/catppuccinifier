use std::env;
use std::path::PathBuf;
use std::process::Command;
use clap::{command, ArgAction, Arg, value_parser};
use clap::parser::ValuesRef;


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
    noise: &str,
    image: &str,
    home_folder_path: &str,
    exec_folder_path: &str,
) {
    println!("Generating {} image with noise level {}", flavor, noise);

    let command = format!(
        "convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/{}.png' -hald-clut '{}-{}'",
        exec_folder_path, image, home_folder_path, noise, flavor, flavor, image
    );

    Command::new("/bin/sh")
        .arg("-c")
        .arg(&command)
        .output()
        .expect("ERROR: Couldn't convert image");
}


fn main() {
    let matches = get_cli().get_matches();
    let flavors_reference = matches.get_many::<String>("flavor");
    let noise_level = matches.get_one::<String>("noise");
    let image = matches.get_one::<String>("image");


    match env::current_dir() {
        Ok(exec_folder) => {
            match dirs::home_dir() {
                Some(home_folder) => {
                    let exec_folder_path = exec_folder.to_str().unwrap();
                    let home_folder_path = home_folder.to_str().unwrap();

                    match flavors_reference {
                        Some(flavors) => {
                            for flavor in flavors {
                                if flavor != "all" {
                                    convert_image(
                                        flavor,
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );
                                }

                                if flavor == "all" {
                                    println!("Generating images with all flavors with noise level {}", noise_level.unwrap());

                                    convert_image(
                                        "latte",
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );

                                    convert_image(
                                        "frappe",
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );

                                    convert_image(
                                        "macchiato",
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );

                                    convert_image(
                                        "mocha",
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );

                                    convert_image(
                                        "oled",
                                        noise_level.unwrap(),
                                        image.unwrap(),
                                        home_folder_path,
                                        exec_folder_path,
                                    );
                                }
                            }
                        }
                        None => {}
                    }
                }
                None => {
                    println!("ERROR: Couldn't get home folder");
                }
            }
        }
        Err(_) => {
            println!("ERROR: Couldn't get current folder");
        }
    }
}
