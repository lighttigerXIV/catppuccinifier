use::std::process::Command;
use std::env;
use std::path::Path;

fn main() {

    match dirs::home_dir() {
        Some(home_folder)=>{
            match env::current_dir() {
                Ok(exec_folder)=>{

                    let home_folder_path = home_folder.to_str().unwrap();
                    let exec_folder_path = exec_folder.to_str().unwrap();
                    let catppuccinifier_folder_path = format!("{}/.local/share/catppuccinifier", home_folder_path);
                    let catppuccinfier_folder = Path::new(catppuccinifier_folder_path.as_str());

                    //Installs binaries
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("sudo cp '{}/installation-files/catppuccinifier' '{}/installation-files/catppuccinifier-gui' /usr/bin/", exec_folder_path, exec_folder_path))
                        .output()
                        .expect("ERROR: Couldn't copy catppuccinifier");

                    //Creates the catppuccinifier folder with the flavors
                    if !catppuccinfier_folder.exists(){

                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(format!("mkdir -p '{}/.local/share/catppuccinifier'", home_folder_path))
                            .output()
                            .expect("ERROR: Couldn't create catppuccinifier folder");
                    }

                    //Copies the flavors to the folder
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("cp -p -r '{}/installation-files/flavors/' '{}/.local/share/catppuccinifier/flavors/'", exec_folder_path, home_folder_path))
                        .output()
                        .expect("ERROR: Couldn't copy flavors");

                    //Copies the icon to the apps folder
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("sudo cp '{}/installation-files/catppuccinifier.png' /usr/share/pixmaps/", exec_folder_path))
                        .output()
                        .expect("ERROR: Couldn't copy icon");


                    //Copies the desktop file
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg(format!("sudo desktop-file-install -m 644 --dir /usr/share/applications/ {}/installation-files/Catppuccinifier.desktop", exec_folder_path))
                        .output()
                        .expect("ERROR: Couldn't copy desktop file");


                    println!("Install completed :P");
                },
                Err(_)=>{
                    println!("ERROR: Couldn't get current folder");
                }
            }
        },
        None =>{
            println!("ERROR: Couldn't get home folder");
        }
    }
}
