use std::{fmt, path::PathBuf};

use catppuccinifier_rs::generation;
use clap::{Parser, ValueEnum};

#[derive(Parser, Clone, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, num_args(0..=5), default_value = "all")]
    pub flavor: Vec<Flavor>,

    #[arg(required = true, help = "The image to catppuccinify")]
    pub image: PathBuf,

    #[arg(
        short = 'd',
        long = "dir",
        help = "The directory where to save images. If nothing is provided it will generate in the same folder as the image"
    )]
    pub output_dir: Option<PathBuf>,

    #[arg(long, default_value_t = 8, value_parser = clap::value_parser!(u8).range(2..=16))]
    pub hald: u8,

    #[arg(short, long, default_value_t = 1.0, value_parser = clap::value_parser!(f64))]
    pub luminosity: f64,

    #[arg(short, long, default_value = "gaussian-rbf")]
    pub algorithm: Algorithm,

    #[arg(long, default_value_t = 96.0)]
    pub shape: f64,

    #[arg(long)]
    pub nearest: Option<usize>,

    #[arg(long, default_value_t = 0.0)]
    pub mean: f64,

    #[arg(long, default_value_t = 20.0)]
    pub std: f64,

    #[arg(long, default_value_t = 512)]
    pub iterations: usize,

    #[arg(long, default_value_t = 4.0)]
    pub power: f64,
}

#[derive(Default, Clone, Debug, ValueEnum)]
pub enum Algorithm {
    ShepardsMethod,
    #[default]
    GaussianRBF,
    LinearRBF,
    GaussianSampling,
    NearestNeighbor,
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq)]
pub enum Flavor {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
    Oled,
    All,
}

impl fmt::Display for Flavor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Flavor::Latte => "latte",
                Flavor::Frappe => "frappe",
                Flavor::Macchiato => "macchiato",
                Flavor::Mocha => "mocha",
                Flavor::Oled => "oled",
                Flavor::All => "all",
            }
        )
    }
}

impl Flavor {
    pub fn as_gen_flavor(&self) -> generation::Flavor {
        match self {
            Flavor::Latte => generation::Flavor::Latte,
            Flavor::Frappe => generation::Flavor::Frappe,
            Flavor::Macchiato => generation::Flavor::Macchiato,
            Flavor::Mocha => generation::Flavor::Mocha,
            _ => generation::Flavor::Oled,
        }
    }
}

impl Algorithm {
    pub fn as_gen_algorithm(&self) -> generation::Algorithm {
        match self {
            Algorithm::ShepardsMethod => generation::Algorithm::ShepardsMethod,
            Algorithm::GaussianRBF => generation::Algorithm::GaussianRBF,
            Algorithm::LinearRBF => generation::Algorithm::LinearRBF,
            Algorithm::GaussianSampling => generation::Algorithm::GaussianSampling,
            Algorithm::NearestNeighbor => generation::Algorithm::NearestNeighbor,
        }
    }
}
