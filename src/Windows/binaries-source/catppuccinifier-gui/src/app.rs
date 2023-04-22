#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use eframe::{egui};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Catppuccinifier {
    picked_path: Option<String>,

    #[serde(skip)]
    noise_value: i8,
}

impl Default for Catppuccinifier {
    fn default() -> Self {
        Self {
            noise_value: 4,
            picked_path: None,
        }
    }
}

impl Catppuccinifier {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

fn generate_image(
    image_path: &String,
    noise_level: &i8,
    flavor: &str
){

    let command = format!(
        r"magick convert '{}' 'C:\Program Files\Catppuccinifier\flavors\noise-{}\{}.png' -hald-clut 'C:\tmp\catppuccinifier\{}.png'",
        image_path,
        noise_level,
        flavor,
        flavor
    );

    let flavor_command = Command::new("powershell")
        .arg("-Command")
        .arg(&command)
        .output()
        .expect("ERROR: Couldn't convert image");

    if !flavor_command.status.success() {
        println!("An error occurred: {}", String::from_utf8_lossy(&flavor_command.stderr));
    }
}

impl eframe::App for Catppuccinifier {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            noise_value,
            picked_path: _,
        } = self;
        catppuccin_egui::set_theme(&ctx, catppuccin_egui::MOCHA);

        egui::SidePanel::left("properties").show(ctx, |ui| {
            ui.heading("Properties");

            ui.label("Noise level:");
            ui.add(egui::Slider::new(noise_value, 0..=4));

            if ui
                .button("Select Image")
                .on_hover_text("Select an image to use")
                .clicked()
            {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }

            if let Some(picked_path) = &self.picked_path {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            if ui.button("Catppuccinify").clicked() {
                if let Some(picked_path) = &self.picked_path {
                    let noise_level = *noise_value;
                    let image_path = picked_path.to_string();
                    
                    std::thread::spawn(move || {

                        Command::new("powershell")
                            .args(&["-Command", "mkdir -p C:\\tmp\\catppuccinifier\\"])
                            .output()
                            .expect("Error while converting images");

                        Command::new("powershell")
                            .args(&["-Command", "mkdir -p C:\\tmp\\catppuccinifier\\"])
                            .output()
                            .expect("Error while converting images");


                        for flavor in ["latte", "frappe", "macchiato", "mocha", "oled"]{

                            generate_image(
                                &image_path,
                                &noise_level,
                                flavor
                            );
                        }
                    });
                }
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to(
                        "Catppuccinifier",
                        "https://github.com/isabelroses/catppuccinifier",
                    );
                    ui.label(" and ");
                    ui.hyperlink_to("Catppuccin", "https://github.com/catppuccin/catppuccin");
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Catppuccinifier");
            ui.label("A tool to make your images look like catppuccin.");

            ui.horizontal(|ui| {
                if ui
                    .button("Save Latte")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    save_image("latte");
                }
                if ui
                    .button("Save Frappe")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    save_image("frappe");
                }
                if ui
                    .button("Save Macchiato")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    save_image("macchiato");

                }
                if ui
                    .button("Save Mocha")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    save_image("mocha");
                }
                if ui
                    .button("Save OLED")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    save_image("oled");
                }
            });
        });
    }
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn save_image(flavor: &str) {

    match dirs::home_dir() {
        Some(home_folder)=>{

            let home_folder_path = home_folder.to_str().unwrap();

            let command = Command::new("powershell")
                .args(&["-Command", &format!("cp C:\\tmp\\catppuccinifier\\{}.png {}\\Pictures\\{}.png", flavor, home_folder_path, flavor)])
                .output()
                .expect("Error while copying file");

            if !command.status.success(){
                println!("Error while copying file");
            }
        },
        None =>{
            println!("Error getting home folder");
        }
    }
}