use std::env;
use std::path::{Path};
use std::process::Command;
use dirs::home_dir;
use glib::{clone};


fn main() {
    match home_dir() {
        Some(home_folder) => {
            let install = clone!( @strong home_folder => move ||{

                match env::current_dir() {
                    Ok(exec_folder) => {

                        let arguments: Vec<String> = env::args().collect();

                        let home_folder_path = home_folder.to_str().unwrap();
                        let exec_folder_path = exec_folder.to_str().unwrap();
                        let catppuccinifier_folder_path = format!("{}/.local/share/catppuccinifier", home_folder_path);
                        let catppuccinfier_folder = Path::new(catppuccinifier_folder_path.as_str());
                        let icon_folder_path = format!("{}/.local/share/icons/hicolor/512x512/apps/", home_folder_path);
                        let icon_folder = Path::new(icon_folder_path.as_str());


                        //Installs the binaries locally
                        if arguments.iter().any(|argument| argument == "local" || argument == "-l"){

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/bin/catppuccinifier' '{}/.local/bin/'", exec_folder_path, home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy catppuccinifier");

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/bin/catppuccinifier-gui' '{}/.local/bin/'", exec_folder_path, home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy catppuccinifier-gui");

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("sed -i \"s|Exec=.*|Exec={}/.local/bin/catppuccinifier-gui|g\" desktop/Catppuccinifier.desktop", home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't modify desktop file");

                        }

                        //Installs binaries as root
                        else{

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/bin/catppuccinifier' /usr/local/bin/", exec_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy catppuccinifier");

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/bin/catppuccinifier-gui' /usr/local/bin/", exec_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy catppuccinifier-gui");

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("sed -i \"s|Exec=.*|Exec=/usr/local/bin/catppuccinifier-gui|g\" desktop/Catppuccinifier.desktop"))
                                .output()
                                .expect("ERROR: Couldn't modify desktop file");
                        }


                        //Creates the catppuccinifier folder with the flavors
                        if !catppuccinfier_folder.exists(){

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("mkdir -p '{}/.local/share/catppuccinifier'", home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't create catppuccinifier folder");
                        }

                        Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp -p -r '{}/src/flavors/' '{}/.local/share/catppuccinifier/flavors/'", exec_folder_path, home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy flavors");


                        //Copies the icon to the apps folder
                        if !icon_folder.exists(){

                            Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("mkdir -p {}/.local/share/icons/hicolor/512x512/apps/", home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't create icon folder");
                        }

                        Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/images/catppuccinifier.png' '{}/.local/share/icons/hicolor/512x512/apps/'", exec_folder_path, home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't copy icon");


                        //Copies the desktop file
                        Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("sed -i \"s|Icon=.*|Icon={}/.local/share/icons/hicolor/512x512/apps/catppuccinifier.png|g\" desktop/Catppuccinifier.desktop", home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't modify desktop file");

                        Command::new("/bin/sh")
                                .arg("-c")
                                .arg(format!("cp '{}/desktop/Catppuccinifier.desktop' '{}/.local/share/applications/'", exec_folder_path, home_folder_path))
                                .output()
                                .expect("ERROR: Couldn't modify desktop file");
                    },
                    Err(_) => {
                        println!("ERROR: Couldn't get current folder")
                    }
                }
            });

            let uninstall = clone!( @strong home_folder => move || {
                let home_folder_path = home_folder.to_str().unwrap();

                let usr_catppuccinifier_file = Path::new("/usr/local/bin/catppuccinifier");
                let usr_catppuccinifier_gui_file = Path::new("/usr/local/bin/catppuccinifier-gui");

                let local_catppuccinifier_path = format!("{}/.local/bin/catppuccinifier", home_folder_path);
                let local_catppuccinifier_file = Path::new(local_catppuccinifier_path.as_str());

                let local_catppuccinifier_gui_path = format!("{}/.local/bin/catppuccinifier-gui", home_folder_path);
                let local_catppuccinifier_gui_file = Path::new(local_catppuccinifier_gui_path.as_str());

                let catppuccinifier_folder_path = format!("{}/.local/share/catppuccinifier/", home_folder_path);
                let catppuccinifier_folder = Path::new(catppuccinifier_folder_path.as_str());

                let icon_path = format!("{}/.local/share/icons/hicolor/512x512/apps/catppuccinifier.png", home_folder_path);
                let icon_file = Path::new(icon_path.as_str());

                let desktop_file_path = format!("{}/.local/share/applications/Catppuccinifier.desktop", home_folder_path);
                let desktop_file = Path::new(desktop_file_path.as_str());


                //Removes catppuccinifier on /usr/local/bin
                if usr_catppuccinifier_file.exists() && usr_catppuccinifier_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg("sudo rm /usr/local/bin/catppuccinifier")
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier from /usr/local/bin/");
                }

                //Removes catppuccinifier-gui on /usr/local/bin
                if usr_catppuccinifier_gui_file.exists() && usr_catppuccinifier_gui_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg("sudo rm /usr/local/bin/catppuccinifier-gui")
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier-gui from /usr/local/bin/");
                }

                //Removes catppuccinifier on .local/bin
                if local_catppuccinifier_file.exists() && local_catppuccinifier_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("rm {}", local_catppuccinifier_path))
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier from .local/bin/");
                }

                //Removes catppuccinifier-gui on .local/bin
                if local_catppuccinifier_gui_file.exists() && local_catppuccinifier_gui_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("rm {}", local_catppuccinifier_gui_path))
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier-gui from .local/bin/");
                }

                //Removes catppuccinifier folder
                if catppuccinifier_folder.exists() && catppuccinifier_folder.is_dir() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("rm -r {}", catppuccinifier_folder_path))
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier folder");
                }

                //Removes catppuccinifier icon
                if icon_file.exists() && icon_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("rm {}", icon_path))
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier icon");
                }

                //Removes catppuccinifier desktop file
                if desktop_file.exists() && desktop_file.is_file() {
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("rm {}", desktop_file_path))
                        .output()
                        .expect("ERROR: Couldn't delete catppuccinifier desktop file");
                }
            });

            let arguments: Vec<String> = env::args().collect();

            if arguments.iter().any(|argument| argument == "install" || argument == "-i") {
                install();
                println!("Install completed");
            }

            if arguments.iter().any(|argument| argument == "uninstall" || argument == "-un") {
                uninstall();
                println!("Uninstall completed");
            }

            if arguments.iter().any(|argument| argument == "update" || argument == "-up") {

                uninstall();
                install();
                println!("Update completed")
            }
        }
        None => {
            println!("ERROR: Couldn't get home folder")
        }
    }
}
