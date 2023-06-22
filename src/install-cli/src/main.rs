use std::env;


fn main() {
    match env::consts::OS {
        "linux" => {
            #[cfg(target_os = "linux")]
            use colored::Colorize;

            #[cfg(target_os = "linux")]
            match install_linux_version() {
                Ok(()) => {
                    println!("{}", "Catppuccinifier installed successfully :D".green());
                }
                Err(error) => {
                    println!("{}", "Error installing catppuccinifier :d".red());
                    println!("{}", error);
                }
            }
        }
        "windows" =>
        {
            #[cfg(target_os = "windows")]
            match install_windows_version() {
                Ok(()) => {
                    println!("Catppuccinifier installed successfully :D");
                    enter_to_close();
                }
                Err(error) => {
                    println!("Error installing catppuccinifier :d");
                    println!("{}", error);
                    enter_to_close();
                }
            }
        }
        _ => {
            println!("OS not supported ;-; | Catppuccinfier only runs in Linux and Windows")
        }
    }
}

#[cfg(target_os = "linux")]
fn install_linux_version() -> Result<(), String> {
    use std::fs;
    use std::os::unix::prelude::PermissionsExt;
    use std::process::Command;

    let exec_dir_path = match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => return Err("Couldn't get current folder".into()),
    };

    //Gives permissions to run
    fs::set_permissions(
        format!("{}/installation-files/catppuccinifier", exec_dir_path),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    fs::set_permissions(
        format!("{}/installation-files/catppuccinifier-gui", exec_dir_path),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();

    let copy_binaries = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo cp '{}/installation-files/catppuccinifier' '{}/installation-files/catppuccinifier-gui' /usr/bin/", exec_dir_path, exec_dir_path))
        .output()
        .expect("");

    if !copy_binaries.status.success() {
        return Err("Couldn't copy binaries to /usr/bin/".into());
    }

    let copy_icon = Command::new("sh")
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

    let copy_desktop = Command::new("sh")
        .arg("-c")
        .arg(format!("sudo desktop-file-install -m 644 --dir /usr/share/applications/ {}/installation-files/Catppuccinifier.desktop", exec_dir_path))
        .output()
        .expect("");

    if !copy_desktop.status.success() {
        return Err("Couldn't copy desktop file".into());
    }

    return Ok(());
}

#[cfg(target_os = "windows")]
fn install_windows_version() -> Result<(), String> {
    //Creates catppuccinifier folder
    if !Path::new(r"C:\Program Files\Catppuccinifier").exists() {
        match powershell_script::run(include_str!("windows\\create-folder.ps1")) {
            Ok(_) => {}
            Err(_) => return Err("Couldn't create catppuccinifier folder".into()),
        }
    }

    //Copies files to folder
    match powershell_script::run(include_str!("windows\\copy-files.ps1")) {
        Ok(_) => {}
        Err(_) => {
            return Err("Couldn't copy files to folder".into());
        }
    }

    let path_variable = match env::var("path") {
        Ok(path) => path,
        Err(_) => {
            return Err("Couldn't read path variable".into());
        }
    };

    if !path_variable.contains(r"C:\Program Files\Catppuccinifier") {
        match powershell_script::run(include_str!("windows\\set-path.ps1")) {
            Ok(_) => {}
            Err(_) => return Err("Couldn't run add path script".into()),
        }
    }

    return Ok(());
}

#[cfg(target_os = "windows")]
fn enter_to_close() {
    use std::io::{self, BufRead};

    println!("Press enter to close...");
    let mut iterator = io::stdin().lock().lines();
    iterator.next();
}
