use std::env;
use std::path::Path;

fn main() {
    match env::consts::OS {
        "linux" =>
        {
            #[cfg(target_os = "linux")]
            match uninstall_in_linux() {
                Ok(()) => {
                    use colored::Colorize;

                    println!(
                        "{}",
                        "Catppuccinifier uninstalled successfully. Bye bye ;-;".green()
                    );
                    println!(
                        r"
    /\_/\           ___
   = o_o =_______    \ \  
    __^      __(  \.__) )
(@)<_____>__(_____)____/
                "
                    )
                }
                Err(error) => {
                    use colored::Colorize;

                    println!("{}", "Error uninstalling catppuccinifier :d".red());
                    println!("{}", error);
                }
            }
        }
        "windows" =>
        {
            #[cfg(target_os = "windows")]
            match uninstall_in_windows() {
                Ok(()) => {
                    println!("Catppuccinifier uninstalled successfully. Bye bye ;-;");
                    enter_to_close();
                }
                Err(()) => {
                    println!("Error uninstalling catppuccinifier :d");
                    enter_to_close();
                }
            }
        }
        _ => {
            println!("OS not supported ;-; | Catppuccinfier only runs in Linux and Windows");
        }
    }
}

#[cfg(target_os = "linux")]
fn uninstall_in_linux() -> Result<(), String> {
    
    use std::process::Command;

    let home_dir_path = match dirs::home_dir() {
        Some(path) => path.to_str().unwrap().to_string(),
        None => return Err("Couldn't get home folder".into()),
    };

    let catppuccinifier_dir = format!("{}/.local/share/catppuccinifier/", home_dir_path);

    //Removes binaries
    if Path::new("/usr/bin/catppuccinifier").exists()
        && Path::new("/usr/bin/catppuccinifier-gui").exists()
    {
        let remove_binaries = Command::new("/bin/sh")
            .arg("-c")
            .arg("sudo rm /usr/bin/catppuccinifier /usr/bin/catppuccinifier-gui")
            .output()
            .expect("");

        if !remove_binaries.status.success() {
            return Err("Couldn't delete binaries".into());
        }
    }

    //Removes local folder
    if Path::new(catppuccinifier_dir.as_str()).is_dir() {
        let remove_catppuccinifier_dir = Command::new("/bin/sh")
            .arg("-c")
            .arg(format!("rm -r {}", catppuccinifier_dir))
            .output()
            .expect("");

        if !remove_catppuccinifier_dir.status.success() {
            return Err("Couldn't delete catppuccinifier folder".into());
        }
    }

    //Removes icon
    if Path::new("/usr/share/pixmaps/catppuccinifier.png").is_file() {
        let remove_icon = Command::new("/bin/sh")
            .arg("-c")
            .arg("sudo rm /usr/share/pixmaps/catppuccinifier.png")
            .output()
            .expect("");

        if !remove_icon.status.success() {
            return Err("Couldn't delete catppuccinifier icon".into());
        }
    }

    //Removes desktop file
    if Path::new("/usr/share/applications/Catppuccinifier.desktop").is_file() {
        let remove_desktop = Command::new("/bin/sh")
            .arg("-c")
            .arg("sudo rm /usr/share/applications/Catppuccinifier.desktop")
            .output()
            .expect("");

        if !remove_desktop.status.success() {
            return Err("Couldn't delete desktop file".into());
        }
    }

    return Ok(());
}

#[cfg(target_os = "windows")]
fn uninstall_in_windows() -> Result<(), ()> {

    if Path::new("C:\\Program Files\\Catppuccinifier").exists(){
        match powershell_script::run(include_str!("windows\\remove-folders.ps1")) {
            Ok(_)=>{return Ok(())},  
            Err(_)=>{return Err(())}
        }
    } else{
        return Ok(());
    }
}


#[cfg(target_os = "windows")]
fn enter_to_close(){
    use std::io::{self, BufRead};

    println!("Press enter to close...");
    let mut iterator = io::stdin().lock().lines();
    iterator.next();
}