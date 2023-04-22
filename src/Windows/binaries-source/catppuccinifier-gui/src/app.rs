#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
                        use std::process::Command;

                        Command::new("powershell")
                            .args(&["-Command", "mkdir -p C:\\tmp\\catppuccinifier\\"])
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert '{}' $HOME\\.catppuccinifier\\flavors\\noise-{}\\latte.png -hald-clut C:\\tmp\\catppuccinifier\\latte.png", image_path, noise_level);

                        Command::new("powershell")
                            .args(&["-Command", &command])
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert '{}' $HOME\\.catppuccinifier\\flavors\\noise-{}\\frappe.png -hald-clut C:\\tmp\\catppuccinifier\\frappe.png", image_path, noise_level);
                        
                        Command::new("powershell")
                            .args(&["-Command", &command])
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert '{}' $HOME\\.catppuccinifier\\flavors\\noise-{}\\macchiato.png -hald-clut C:\\tmp\\catppuccinifier\\macchiato.png", image_path, noise_level);
                        
                        Command::new("powershell")
                            .args(&["-Command", &command])
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert '{}' $HOME\\.catppuccinifier\\flavors\\noise-{}\\mocha.png -hald-clut C:\\tmp\\catppuccinifier\\mocha.png", image_path, noise_level);
                        
                        Command::new("powershell")
                            .args(&["-Command", &command])
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert '{}' $HOME\\.catppuccinifier\\flavors\\noise-{}\\oled.png -hald-clut C:\\tmp\\catppuccinifier\\oled.png", image_path, noise_level);
                        
                        Command::new("powershell")
                            .args(&["-Command", &command])
                            .output()
                            .expect("Error while converting images");
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
                    copy_file("C:\\tmp\\catppuccinifier\\latte.png", "latte.png");
                }
                if ui
                    .button("Save Frappe")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    copy_file("C:\\tmp\\catppuccinifier\\frappe.png", "frappe.png");
                }
                if ui
                    .button("Save Macchiato")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    copy_file("C:\\tmp\\catppuccinifier\\macchiato.png", "macchiato.png");

                }
                if ui
                    .button("Save Mocha")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    copy_file("C:\\tmp\\catppuccinifier\\mocha.png", "mocha.png");
                }
                if ui
                    .button("Save OLED")
                    .on_hover_text("Select an image to use")
                    .clicked()
                {
                    copy_file("C:\\tmp\\catppuccinifier\\oled.png", "oled.png");
                }
            });
        });
    }
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}

fn copy_file(from: &str, to: &str) {
    std::process::Command::new("powershell")
        .args(&["-Command", &format!("cp {} $HOME\\Pictures\\{}", from, to)])
        .output()
        .expect("Error while copying file");
}