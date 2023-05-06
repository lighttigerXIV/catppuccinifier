use colored::Colorize;
use std::env;

fn main() {
    match env::consts::OS {
        "linux" => match install_linux_version() {
            Ok(()) => {
                println!("{}", "Catppuccinifier installed successfully :D".green());
            }
            Err(error) => {
                println!("{}", "Error installing catppuccinifier :d".red());
                println!("{}", error);
            }
        },
        _ => {
            println!("OS not supported ;-; | Catppuccinfier only runs in Linux and Windows")
        }
    }
}

#[cfg(target_os = "linux")]
fn install_linux_version() -> Result<(), String> {
    use std::fs;
    use std::os::unix::prelude::PermissionsExt;
    use std::path::Path;
    use std::process::Command;

    let home_dir_path = match dirs::home_dir() {
        Some(path) => path.to_str().unwrap().to_string(),
        None => return Err("Couldn't get home path".into()),
    };

    let exec_dir_path = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => return Err("Couldn't get current folder".into()),
    };

    
    //Gives permissions to run
    fs::set_permissions(format!("{}/installation-files/catppuccinifier", exec_dir_path), fs::Permissions::from_mode(0o755)).unwrap();
    fs::set_permissions(format!("{}/installation-files/catppuccinifier-gui", exec_dir_path), fs::Permissions::from_mode(0o755)).unwrap();


    let copy_binaries = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("sudo cp '{}/installation-files/catppuccinifier' '{}/installation-files/catppuccinifier-gui' /usr/bin/", exec_dir_path, exec_dir_path))
        .output()
        .expect("");

    if !copy_binaries.status.success() {
        return Err("Couldn't copy binaries to /usr/bin/".into());
    }

    if !Path::new(format!("{}/.local/share/catppuccinifier", home_dir_path).as_str()).is_dir() {
        match fs::create_dir(format!("{}/.local/share/catppuccinifier", home_dir_path).as_str()) {
            Ok(()) => {}
            Err(_) => return Err("Error creating catppuccinifier directory".into()),
        }
    }

    let copy_flavors = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "cp -p -r '{}/installation-files/flavors/' '{}/.local/share/catppuccinifier/flavors/'",
            exec_dir_path, home_dir_path
        ))
        .output()
        .expect("");

    if !copy_flavors.status.success() {
        return Err("Couldn't copy flavors to catppuccinifier directory".into());
    }

    let copy_icon = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!(
            "sudo cp '{}/installation-files/catppuccinifier.png' /usr/share/pixmaps/",
            exec_dir_path
        ))
        .output()
        .expect("");

    if !copy_icon.status.success() {
        return Err("Couldn't copy icon".into());
    }

    let copy_desktop = Command::new("/bin/sh")
        .arg("-c")
        .arg(format!("sudo desktop-file-install -m 644 --dir /usr/share/applications/ {}/installation-files/Catppuccinifier.desktop", exec_dir_path))
        .output()
        .expect("");

    if !copy_desktop.status.success() {
        return Err("Couldn't copy desktop file".into());
    }

    return Ok(());
}
