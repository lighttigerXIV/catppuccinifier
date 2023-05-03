use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let remove_folders_script = include_str!("remove-folders.ps1");

    if Path::new(r"C:\Program Files\Catppuccinifier").exists() {
        match powershell_script::run(remove_folders_script) {
            Ok(_)=>{
                println!("Catppuccinifier uninstalled successfully");
            },
            Err(_)=>{
                println!("ERROR: Make sure you run the script as administrator");
            }
        }
    } else {
        println!("ERROR: Catppuccinifier folder not found");
    }

    println!("Press enter to close...");
    let mut iterator = io::stdin().lock().lines();
    iterator.next();
}
