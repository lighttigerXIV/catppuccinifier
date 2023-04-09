use std::env;
use std::process::Command;
use substring::Substring;

fn main() {
    match env::current_dir() {
        Ok(current_folder) => {
            let arguments: Vec<String> = env::args().collect();
            let current_folder_path = current_folder.to_str().unwrap();
            let catppuccinifier_folder_path = r"C:\Program Files\Catppuccinifier";

            let mut noise_level = 0;

            //Gets noise level
            if arguments
                .iter()
                .any(|argument| argument == "n1" || argument == "noise1")
            {
                noise_level = 1;
            }

            if arguments
                .iter()
                .any(|argument| argument == "n2" || argument == "noise2")
            {
                noise_level = 2;
            }

            if arguments
                .iter()
                .any(|argument| argument == "n3" || argument == "noise3")
            {
                noise_level = 3;
            }

            if arguments
                .iter()
                .any(|argument| argument == "n4" || argument == "noise4")
            {
                noise_level = 4;
            }

            let mut image = String::from(&arguments[arguments.len() - 1]);

            if image.contains(r".\"){
                image = image.substring(2, image.len()).to_string();
            }

            let mut i = 0;

            while i < arguments.len() {
                let argument = &arguments[i];

                if argument == "latte" || argument == "-l" {
                    println!("Generating latte image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\latte.png' -hald-clut '{}\latte-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to latte");
                }

                if argument == "frappe" || argument == "-f" {
                    println!("Generating frappe image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\frappe.png' -hald-clut '{}\frappe-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to frappe");
                }

                if argument == "macchiato" || argument == "-ma" {
                    println!(
                        "Generating macchiato image with noise level {}",
                        noise_level
                    );

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\macchiato.png' -hald-clut '{}\macchiato-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to macchiato");
                }

                if argument == "mocha" || argument == "-mo" {
                    println!("Generating mocha image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\mocha.png' -hald-clut '{}\mocha-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to mocha");
                }

                if argument == "oled" || argument == "-o" {
                    println!("Generating oled image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\oled.png' -hald-clut '{}\oled-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to oled");
                }

                if argument == "all" || argument == "-a" {
                    println!("Generating latte image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\latte.png' -hald-clut '{}\latte-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to latte");

                    println!("Generating frappe image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\frappe.png' -hald-clut '{}\frappe-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to frappe");

                    println!(
                        "Generating macchiato image with noise level {}",
                        noise_level
                    );

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\macchiato.png' -hald-clut '{}\macchiato-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to macchiato");

                    println!("Generating mocha image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\mocha.png' -hald-clut '{}\mocha-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to mocha");

                    println!("Generating oled image with noise level {}", noise_level);

                    let command = format!(
                        r"magick convert '{}\{}' '{}\flavors\noise-{}\oled.png' -hald-clut '{}\oled-{}'",
                        current_folder_path,
                        image,
                        catppuccinifier_folder_path,
                        noise_level,
                        current_folder_path,
                        image
                    );

                    Command::new("powershell")
                        .arg("-Command")
                        .arg(&command)
                        .output()
                        .expect("ERROR: Couldn't convert image to oled");
                }

                i += 1;
            }
        }
        Err(_) => {
            println!("ERROR: Couldn't get current folder");
        }
    }
}
