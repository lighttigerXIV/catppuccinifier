use std::{env};
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    match env::var("path") {
        Ok(path_variable) => {

            let add_path_script = include_str!("set-path.ps1");
            let create_folder_script = include_str!("create-folder.ps1");
            let copy_files_script = include_str!("copy-files.ps1");

            let mut install_successful = true;

            //Creates catppuccinifier folder
            if !Path::new(r"C:\Program Files\Catppuccinifier").exists() {
                match powershell_script::run(create_folder_script) {
                    Ok(_) => {

                        //Copies the installation files to catppuccinifier folder
                        match powershell_script::run(copy_files_script) {
                            Ok(_) => {

                                //Adds the folder to path environment variable
                                if !path_variable.contains(r"C:\Program Files\Catppuccinifier") {
                                    match powershell_script::run(add_path_script) {
                                        Ok(_) => {}
                                        Err(_) => {
                                            install_successful = false;
                                            println!("ERROR: Couldn't run add path script.")
                                        }
                                    }
                                }
                            }
                            Err(error) => {
                                install_successful = false;
                                println!("ERROR: Couldn't copy files to catppuccinifier folder. {}", error.to_string())
                            }
                        }
                    }
                    Err(_) => {
                        install_successful = false;
                        println!("ERROR: Couldn't create catppuccinifier folder");
                    }
                }
            }

            if install_successful {
                println!("Catppuccinifier installed successfully");
            } else {
                println!("ERROR: Couldn't install Catppuccinifier. Make sure you run the script as administrator");
            }


            println!("Press enter to close...");
            let mut iterator = io::stdin().lock().lines();
            iterator.next();
        }
        Err(_) => {}
    }
}
