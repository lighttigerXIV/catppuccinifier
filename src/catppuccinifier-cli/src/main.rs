use catppuccinifier_rs::catppuccinify;
use catppuccinifier_rs::generation::Properties;
use clap::Parser;
use cli::{Algorithm, Cli, Flavor};
use colored::Colorize;
use inquire::Confirm;
use std::env;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

pub mod cli;

fn compatible_image<T: AsRef<Path>>(image: T) -> bool {
    let image = image.as_ref();
    let extension = if let Some(extension) = image.extension() {
        extension.to_str().unwrap()
    } else {
        ""
    };

    ["jpg", "jpeg", "png", "webp"].contains(&extension)
}

pub fn get_output_path<P: AsRef<Path>>(
    target_path: P,
    output_dir: &Option<PathBuf>,
    flavor: &Flavor,
) -> PathBuf {
    let target_path = target_path.as_ref();
    let parent = if let Some(output_dir) = output_dir {
        output_dir.to_str().unwrap()
    } else {
        target_path.parent().unwrap().to_str().to_owned().unwrap()
    };

    let stem = target_path.file_stem().unwrap().to_str().unwrap();
    let extension = target_path.extension().unwrap().to_str().unwrap();
    let flavor_str: String = format!("{flavor}");

    let full_path = format!("{parent}/{stem}-{flavor_str}.{extension}");
    println!("path: {:?}", &full_path);
    PathBuf::from(&full_path)
}

pub fn get_properties(cli: &Cli) -> Properties {
    Properties {
        hald_level: cli.hald,
        luminosity: cli.luminosity,
        algorithm: cli.algorithm.as_gen_algorithm(),
        shape: cli.shape,
        nearest: if let Some(nearest) = cli.nearest {
            nearest
        } else {
            match cli.algorithm {
                Algorithm::LinearRBF => 5,
                _ => 26,
            }
        },
        mean: cli.mean,
        std: cli.std,
        iterations: cli.iterations,
        power: cli.power,
    }
}

pub fn generate_image<P: AsRef<Path>>(
    target_path: P,
    flavor: &Flavor,
    properties: &Properties,
    output_path: P,
) {
    println!("{}", format!("✨ Generating {flavor} image"));

    match catppuccinify(
        properties,
        &flavor.as_gen_flavor(),
        target_path.as_ref(),
        output_path.as_ref(),
    ) {
        Ok(_) => {
            println!("✅ Image successfully generated\n")
        }
        Err(e) => println!(
            "{}",
            format!("🙀 An error has happened while generating image. {}", e).red()
        ),
    }
}

fn main() {
    let cli = Cli::parse();

    let mut target_path = env::current_dir().unwrap().join(cli.image.clone());
    let flavors = &cli.flavor;

    if !target_path.exists() {
        println!("{}", "Invalid path".red());
        exit(1);
    }

    if !target_path.is_file() {
        println!("{}", "Image path isn't a file".red());
        exit(1);
    }

    if !compatible_image(&target_path) {
        println!(
            "{}",
            "Invalid image.\nCompatible types: [jpg, png, webp]".red(),
        );

        let convert = Confirm::new("Would you like to convert the image? (Requires image magick)")
            .with_default(true)
            .prompt()
            .unwrap();

        if convert {
            let mut new_path = target_path.to_owned();
            new_path.set_extension(".png");

            let target_str: String = target_path
                .to_owned()
                .into_os_string()
                .into_string()
                .unwrap();

            let output_str: String = new_path.to_owned().into_os_string().into_string().unwrap();

            Command::new("magick")
                .args([&target_str, &output_str])
                .output()
                .unwrap();

            target_path = new_path;
        } else {
            exit(1);
        }
    }

    if let Some(output_dir) = &cli.output_dir {
        if !output_dir.exists() {
            create_dir_all(&output_dir).unwrap();
        }
    }

    let properties = get_properties(&cli);

    if flavors.iter().any(|f| f == &Flavor::All) {
        generate_image(
            &target_path,
            &Flavor::Latte,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &Flavor::Latte),
        );

        generate_image(
            &target_path,
            &Flavor::Frappe,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &Flavor::Frappe),
        );

        generate_image(
            &target_path,
            &Flavor::Macchiato,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &Flavor::Macchiato),
        );

        generate_image(
            &target_path,
            &Flavor::Mocha,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &Flavor::Mocha),
        );

        generate_image(
            &target_path,
            &Flavor::Oled,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &Flavor::Oled),
        );

        exit(0);
    }

    for flavor in flavors {
        generate_image(
            &target_path,
            &flavor,
            &properties,
            &&get_output_path(&target_path, &cli.output_dir, &flavor),
        );
    }
}
