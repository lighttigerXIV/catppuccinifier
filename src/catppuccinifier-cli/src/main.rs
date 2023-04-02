use std::env;
use std::process::Command;



fn main() {
    match env::current_dir() {
        Ok(exec_folder) => {

            match dirs::home_dir() {
                Some(home_folder) =>{

                    let arguments: Vec<String> = env::args().collect();
                    let exec_folder_path = exec_folder.to_str().unwrap();
                    let home_folder_path = home_folder.to_str().unwrap();

                    let mut noise_level = 0;

                    //Gets noise level
                    if arguments.iter().any(|argument| argument == "n1" || argument == "noise1"){
                        noise_level = 1;
                    }

                    if arguments.iter().any(|argument| argument == "n2" || argument == "noise2"){
                        noise_level = 2;
                    }

                    if arguments.iter().any(|argument| argument == "n3" || argument == "noise3"){
                        noise_level = 3;
                    }

                    if arguments.iter().any(|argument| argument == "n4" || argument == "noise4"){
                        noise_level = 4;
                    }

                    let image = &arguments[arguments.len() - 1];

                    let mut i = 0;

                    while i < arguments.len(){

                        let argument = &arguments[i];

                        if argument == "latte" || argument == "-l"{

                            println!("Generating latte image with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/latte.png' -hald-clut 'latte-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to latte");
                        }

                        if argument == "frappe" || argument == "-f"{

                            println!("Generating frappe image with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/frappe.png' -hald-clut 'frappe-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to frappe");
                        }

                        if argument == "macchiato" || argument == "-ma"{

                            println!("Generating macchiato image with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/macchiato.png' -hald-clut 'macchiato-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to macchiato");
                        }

                        if argument == "mocha" || argument == "-mo"{

                            println!("Generating mocha image with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/mocha.png' -hald-clut 'mocha-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to mocha");
                        }

                        if argument == "oled" || argument == "-o"{

                            println!("Generating oled image with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/oled.png' -hald-clut 'oled-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to oled");
                        }

                        if argument == "all" || argument == "-a"{

                            println!("Generating images with all flavors with noise level {}", noise_level);

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/latte.png' -hald-clut 'latte-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to latte");

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/frappe.png' -hald-clut 'frappe-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to frappe");

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/macchiato.png' -hald-clut 'macchiato-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to macchiato");

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/mocha.png' -hald-clut 'mocha-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to mocha");

                            let command = format!("convert '{}/{}' '{}/.local/share/catppuccinifier/flavors/noise-{}/oled.png' -hald-clut 'oled-{}'", exec_folder_path, image, home_folder_path, noise_level, image);

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(&command)
                                .output()
                                .expect("ERROR: Couldn't convert image to oled");
                        }

                        i += 1;
                    }
                },
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
