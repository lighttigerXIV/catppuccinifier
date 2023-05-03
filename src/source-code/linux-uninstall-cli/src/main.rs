use std::path::Path;
use std::process::Command;

fn main() {

    match dirs::home_dir() {
        Some(home_folder)=>{

            let home_folder_path = home_folder.to_str().unwrap();
            let usr_catppuccinifier_file = Path::new("/usr/bin/catppuccinifier");
            let catppuccinifier_folder_path = format!("{}/.local/share/catppuccinifier/", home_folder_path);
            let catppuccinifier_folder = Path::new(catppuccinifier_folder_path.as_str());
            let icon_file = Path::new("/usr/share/pixmaps/catppuccinifier.png");
            let desktop_file = Path::new("/usr/share/applications/Catppuccinifier.desktop");

            //Removes catppuccinifier on /usr/local/bin
            if usr_catppuccinifier_file.exists() && usr_catppuccinifier_file.is_file() {
                Command::new("/bin/sh")
                    .arg("-c")
                    .arg("sudo rm /usr/bin/catppuccinifier /usr/bin/catppuccinifier-gui")
                    .output()
                    .expect("ERROR: Couldn't delete catppuccinifier");
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
                    .arg("sudo rm /usr/share/pixmaps/catppuccinifier.png")
                    .output()
                    .expect("ERROR: Couldn't delete catppuccinifier icon");
            }

            //Removes catppuccinifier desktop file
            if desktop_file.exists() && desktop_file.is_file() {
                Command::new("/bin/sh")
                    .arg("-c")
                    .arg("sudo rm /usr/share/applications/Catppuccinifier.desktop")
                    .output()
                    .expect("ERROR: Couldn't delete catppuccinifier desktop file");
            }

            println!("Uninstall completed :d")
        },
        None =>{
            println!("ERROR: Couldn't get home folder");
        }
    }
}
