use catppuccinifier_rs::generate::{generate_image, GenerateProperties};
use clap::{command, Parser, ValueEnum};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, num_args(0..=5), default_value = "all")]
    flavor: Vec<Flavor>,

    #[arg(short, long, required = true)]
    image: PathBuf,

    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(2..=16))]
    hald: u8,

    #[arg(short, long, default_value_t = 1.0, value_parser = clap::value_parser!(f64))]
    luminosity: f64,

    #[arg(short, long, default_value = "gaussian-rbf")]
    algorithm: Algorithm,

    #[arg(long, default_value_t = 96.0)]
    shape: f64,

    #[arg(long)]
    nearest: Option<usize>,

    #[arg(long, default_value_t = 0.0)]
    mean: f64,

    #[arg(long, default_value_t = 20.0)]
    std: f64,

    #[arg(long, default_value_t = 512)]
    iterations: usize,

    #[arg(long, default_value_t = 4.0)]
    power: f64,
}

#[derive(Default, Clone, Debug, ValueEnum)]
enum Algorithm {
    ShepardsMethod,
    #[default]
    GaussianRBF,
    LinearRBF,
    GaussianSampling,
    NearestNeighbor,
}

#[derive(Clone, Debug, ValueEnum, PartialEq)]
enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Oled,
    All,
}

fn main() {
    let cli = Cli::parse();
    let image_path = cli.image;
    let flavors = cli.flavor;
    let hald_level = cli.hald;
    let luminosity = cli.luminosity;
    let algorithm = cli.algorithm;
    let shape = cli.shape;
    let nearest = cli.nearest.unwrap_or(match &algorithm {
        Algorithm::LinearRBF => 5,
        _ => 26,
    });
    let mean = cli.mean;
    let std = cli.std;
    let iterations = cli.iterations;
    let power = cli.power;

    let temp_path = match env::consts::OS {
        "linux" => "/tmp/catppuccinifier/".to_string(),
        "windows" => "C:\\Windows\\TEMP\\catppuccinifier\\".to_string(),
        _ => "".to_string(),
    };

    if !Path::new(&temp_path).exists() {
        fs::create_dir(&temp_path).expect("");
    }

    if !Path::new(&image_path.to_owned()).exists() {
        println!("Error. Couldn't find image");
        std::process::exit(1);
    }

    let image_extension = match Path::new(&image_path.to_owned()).extension() {
        Some(extension) => extension.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    let image_name = format!(
        "{}.{}",
        image_path
            .to_owned()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap(),
        image_extension.to_owned()
    );

    if !["jpg", "jpeg", "png", "webp"].contains(&image_extension.as_str()) {
        println!("Error. File not supported");
        std::process::exit(1);
    }

    for flavor in flavors {
        match flavor {
            Flavor::All => {
                let possible_flavors = [
                    Flavor::Latte,
                    Flavor::Frappe,
                    Flavor::Macchiato,
                    Flavor::Mocha,
                    Flavor::Oled,
                ];

                for possible_flavor in possible_flavors {
                    let algorithm = match algorithm.clone() {
                        Algorithm::ShepardsMethod => {
                            catppuccinifier_rs::generate::Algorithm::ShepardsMethod
                        }
                        Algorithm::GaussianRBF => {
                            catppuccinifier_rs::generate::Algorithm::GaussianRBF
                        }
                        Algorithm::LinearRBF => catppuccinifier_rs::generate::Algorithm::LinearRBF,
                        Algorithm::GaussianSampling => {
                            catppuccinifier_rs::generate::Algorithm::GaussianSampling
                        }
                        Algorithm::NearestNeighbor => {
                            catppuccinifier_rs::generate::Algorithm::NearestNeighbor
                        }
                    };

                    let flavor = if possible_flavor.to_owned() == Flavor::Latte {
                        catppuccinifier_rs::generate::Flavor::Latte
                    } else if possible_flavor.to_owned() == Flavor::Frappe {
                        catppuccinifier_rs::generate::Flavor::Frappe
                    } else if possible_flavor.to_owned() == Flavor::Macchiato {
                        catppuccinifier_rs::generate::Flavor::Macchiato
                    } else if possible_flavor.to_owned() == Flavor::Mocha {
                        catppuccinifier_rs::generate::Flavor::Mocha
                    } else {
                        catppuccinifier_rs::generate::Flavor::Oled
                    };

                    let mut save_path = image_path.parent().unwrap().to_owned();
                    save_path.push(format!(
                        "{}-{}-{}",
                        format!("{:?}", flavor),
                        hald_level.to_owned(),
                        image_name
                    ));

                    match generate_image(
                        GenerateProperties {
                            hald_level,
                            luminosity,
                            algorithm,
                            shape,
                            nearest,
                            mean,
                            std,
                            iterations,
                            power,
                        },
                        flavor,
                        image_path,
                        save_path,
                    ){
                        Ok(())=> {println!("Image Generated Successfully")},
                        Err(e)=> {println!("{}", e)}
                    }
                }
            }
            _ => {
                let algorithm = match algorithm.clone() {
                        Algorithm::ShepardsMethod => {
                            catppuccinifier_rs::generate::Algorithm::ShepardsMethod
                        }
                        Algorithm::GaussianRBF => {
                            catppuccinifier_rs::generate::Algorithm::GaussianRBF
                        }
                        Algorithm::LinearRBF => catppuccinifier_rs::generate::Algorithm::LinearRBF,
                        Algorithm::GaussianSampling => {
                            catppuccinifier_rs::generate::Algorithm::GaussianSampling
                        }
                        Algorithm::NearestNeighbor => {
                            catppuccinifier_rs::generate::Algorithm::NearestNeighbor
                        }
                    };

                    let mut image_flavor = if flavor.to_owned() == Flavor::Latte {
                        catppuccinifier_rs::generate::Flavor::Latte
                    } else if flavor.to_owned() == Flavor::Frappe {
                        catppuccinifier_rs::generate::Flavor::Frappe
                    } else if flavor.to_owned() == Flavor::Macchiato {
                        catppuccinifier_rs::generate::Flavor::Macchiato
                    } else if flavor.to_owned() == Flavor::Mocha {
                        catppuccinifier_rs::generate::Flavor::Mocha
                    } else {
                        catppuccinifier_rs::generate::Flavor::Oled
                    };

                    let mut save_path = image_path.parent().unwrap().to_owned();
                    save_path.push(format!(
                        "{}-{}-{}",
                        format!("{:?}", flavor),
                        hald_level.to_owned(),
                        image_name
                    ));

                    match generate_image(
                        GenerateProperties {
                            hald_level,
                            luminosity,
                            algorithm,
                            shape,
                            nearest,
                            mean,
                            std,
                            iterations,
                            power,
                        },
                        image_flavor,
                        image_path,
                        save_path,
                    ){
                        Ok(())=> {println!("Image Generated Successfully")},
                        Err(e)=> {println!("{}", e)}
                    }
            }
        }
    }
}
