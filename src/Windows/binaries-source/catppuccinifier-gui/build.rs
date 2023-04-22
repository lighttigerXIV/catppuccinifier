#[cfg(windows)]
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("..\\..\\installation-files\\catppuccinifier.ico");
    res.compile().unwrap();

    std::process::Command::new("powershell")
        .args(&["-Command", "mkdir -p $HOME\\.catppuccinifier"])
        .output()
        .expect("Failed to install .catppuccinifier folder");

    std::process::Command::new("powershell")
        .args(&[
            "-Command",
            "Copy-Item -Path ..\\..\\installation-files\\flavors -Destination $HOME\\.catppuccinifier\\ -Recurse -Force",
        ])
        .output()
        .expect("Failed to install .catppuccinifier\\flavors folder");
    std::process::Command::new("powershell")
        .args(&[
            "-Command",
            "Copy-Item -Path ..\\..\\installation-files\\catppuccinifier.png -Destination $HOME\\.catppuccinifier\\catppuccinifier.png -Force",
        ])
        .output()
        .expect("Failed to install catppuccinifier.png");
}
