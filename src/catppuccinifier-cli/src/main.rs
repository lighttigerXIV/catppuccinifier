use clap::{command, Parser, ValueEnum};
use exoquant::SimpleColorSpace;
use image::open;
use lutgen::identity::correct_image;
use lutgen::interpolation::{
    GaussianRemapper, GaussianSamplingRemapper, LinearRemapper, NearestNeighborRemapper,
    ShepardRemapper,
};
use lutgen::{GenerateLut, Palette};
use std::path::{Path, PathBuf};
use std::{env, fs};
use substring::Substring;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, num_args(0..=5), default_value = "all")]
    flavor: Vec<Flavor>,

    #[arg(short, long, required = true)]
    image: PathBuf,

    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(2..=16))]
    hald: u8,

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

#[derive(Clone, Debug, ValueEnum)]
enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Oled,
    All,
}

fn generate(
    temp_path: String,
    image_path: String,
    image_folder: String,
    image_name: String,
    hald_level: u8,
    flavor: Flavor,
    image_extension: String,
    algorithm: Algorithm,
    shape: f64,
    nearest: usize,
    mean: f64,
    std: f64,
    iterations: usize,
    power: f64,
) -> Result<String, String> {
    println!("Generating {:?}", &flavor);

    const SEED: u64 = u64::from_be_bytes(*b"42080085");

    let palette = match flavor {
        Flavor::Latte => Palette::CatppuccinLatte.get(),
        Flavor::Frappe => Palette::CatppuccinFrappe.get(),
        Flavor::Macchiato => Palette::CatppuccinMacchiato.get(),
        Flavor::Mocha => Palette::CatppuccinMocha.get(),
        _ => Palette::CatppuccinOled.get(),
    };

    let hald_clut = match algorithm {
        Algorithm::GaussianRBF => {
            GaussianRemapper::new(&palette, shape, nearest).generate_lut(hald_level)
        }

        Algorithm::GaussianSampling => GaussianSamplingRemapper::new(
            &palette,
            mean,
            std,
            iterations,
            SEED,
            SimpleColorSpace::default(),
        )
        .generate_lut(hald_level),

        Algorithm::LinearRBF => LinearRemapper::new(&palette, nearest).generate_lut(hald_level),

        Algorithm::ShepardsMethod => {
            ShepardRemapper::new(&palette, power, nearest).generate_lut(hald_level)
        }

        _ => NearestNeighborRemapper::new(&palette, SimpleColorSpace::default())
            .generate_lut(hald_level),
    };

    let lut_was_generated = match hald_clut.save(format!("{}lut.png", temp_path)) {
        Err(_) => false,
        Ok(_) => true,
    };

    if lut_was_generated {
        let mut new_image = open(image_path).unwrap().to_rgb8();
        correct_image(&mut new_image, &hald_clut);

        let save_path = match env::consts::OS {
            "linux" => format!(
                "{}/{}-hald{}-{:?}.{}",
                image_folder, image_name, &hald_level, &flavor, &image_extension
            ),
            "windows" => format!(
                "{}\\{}-hald{}-{:?}.{}",
                image_folder, image_name, &hald_level, &flavor, &image_extension
            ),
            _ => "".to_string(),
        };

        match new_image.save(&save_path) {
            Ok(_) => return Ok(save_path.into()),
            Err(error) => return Err(error.to_string()),
        };
    } else {
        return Err("Couldn't generate LUT".into());
    }
}

fn main() {
    let cli = Cli::parse();
    let image_path = cli.image.to_str().unwrap();
    let image_folder = cli.image.parent().unwrap();
    let flavors = cli.flavor;
    let hald_level = cli.hald;
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

    if !Path::new(&image_path).exists() {
        println!("Error. Couldn't find image");
        std::process::exit(1);
    }

    let image_extension = match Path::new(&image_path).extension() {
        Some(extension) => extension.to_str().unwrap().to_string(),
        None => "".to_string(),
    };

    let image_name = cli
        .image
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .replace(format!(".{}", &image_extension).as_str(), "");

    if !["jpg", "jpeg", "png", "webp"].contains(&image_extension.as_str()) {
        println!("Error. File not supported");
        std::process::exit(1);
    }

    let image = if image_path.contains(r".\") {
        image_path.substring(2, image_path.len())
    } else {
        image_path
    };

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
                    match generate(
                        temp_path.clone(),
                        image.to_string(),
                        image_folder.to_str().unwrap().to_string(),
                        image_name.clone(),
                        hald_level,
                        possible_flavor.clone(),
                        image_extension.clone(),
                        algorithm.clone(),
                        shape,
                        nearest,
                        mean,
                        std,
                        iterations,
                        power,
                    ) {
                        Ok(path) => {
                            println!("Successfully generated image: {}", &path);
                        }
                        Err(error) => {
                            println!("Error generating image: {}", error);
                        }
                    };
                }
            }
            _ => {
                match generate(
                    temp_path.clone(),
                    image.to_string(),
                    image_folder.to_str().unwrap().to_string(),
                    image_name.clone(),
                    hald_level,
                    flavor.clone(),
                    image_extension.clone(),
                    algorithm.clone(),
                    shape,
                    nearest,
                    mean,
                    std,
                    iterations,
                    power,
                ) {
                    Ok(path) => {
                        println!("Successfully generated image: {}", &path);
                    }
                    Err(error) => {
                        println!("Error generating image: {}", error);
                    }
                };
            }
        }
    }
}
