mod items;
mod css;


use std::cell::RefCell;
use adw::{prelude::*, Application, ApplicationWindow, HeaderBar};

use gdk_pixbuf::Pixbuf;
use glib::{clone, MainContext, PRIORITY_DEFAULT};
use std::rc::Rc;
use std::process::Command;
use std::thread;
use gtk4::{Align, Button, Image, Orientation, Scale, ScrolledWindow, STYLE_PROVIDER_PRIORITY_APPLICATION, FileChooserDialog, FileChooserAction, ResponseType, FileFilter, Grid, ProgressBar, GestureClick, MenuButton, Popover, Label, STYLE_PROVIDER_PRIORITY_USER};
use crate::css::app_css;
use crate::items::{accent_column, column, label, preview_button, row, save_button, small_horizontal_spacer, small_vertical_spacer, title};

#[tokio::main]
async fn main() {
    let app = Application::builder()
        .application_id("com.lighttigerxiv.catppuccinifier")
        .build();


    app.connect_activate(|app| {
        let app_column = column();
        let header_bar = HeaderBar::new();

        let menu_button = MenuButton::new();
        menu_button.set_icon_name("open-menu-symbolic");

        let popover = Popover::new();

        let popover_column = column();

        let about_item = Label::new(Some("About"));
        about_item.add_css_class("clickable");
        about_item.style_context().add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

        popover_column.append(&about_item);
        popover.set_autohide(true);

        popover.set_child(Some(&popover_column));
        menu_button.set_popover(Some(&popover));

        header_bar.pack_end(&menu_button);

        app_column.append(&header_bar);
        let content_scroll_box = ScrolledWindow::new();

        //Properties
        let properties_column = accent_column();
        properties_column.set_vexpand(true);
        properties_column.set_hexpand(true);
        properties_column.add_css_class("small-padding");

        let noise_scale_row = row();
        let noise_scale = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
        let noise_level_label = label("4");

        noise_scale.set_digits(0);
        noise_scale.set_hexpand(true);
        noise_scale.set_value(4.0);

        noise_scale_row.append(&noise_scale);
        noise_scale_row.append(&small_horizontal_spacer());
        noise_scale_row.append(&noise_level_label);

        let select_image_button = Button::with_label("Select Image");
        select_image_button.add_css_class("round-button");
        select_image_button.style_context()
            .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_USER);


        let selected_image_path = Rc::new(RefCell::new("".to_string()));

        let selected_image_name = Rc::new(RefCell::new("".to_string()));

        let selected_image = Image::new();
        selected_image.set_visible(false);
        selected_image.set_size_request(-1, 300);

        let show_generate_button = false;

        let generate_images_button = Button::with_label("Generate Images");
        generate_images_button.add_css_class("round-button");
        generate_images_button.style_context()
            .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_USER);
        generate_images_button.set_visible(show_generate_button);

        //Results
        let results_column = column();
        results_column.set_hexpand(true);
        results_column.add_css_class("small-padding");
        results_column.style_context()
            .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

        let results_title = title("Results");
        results_title.set_visible(false);

        let image_results_column = column();


        //Generated Latte Column
        let latte_column = accent_column();
        latte_column.add_css_class("generated-column");
        latte_column.set_margin_bottom(8);
        latte_column.set_margin_end(8);

        let latte_progressbar = ProgressBar::new();
        latte_progressbar.set_pulse_step(10000.0);
        latte_progressbar.pulse();
        latte_progressbar.set_visible(false);

        let latte_image = Image::new();
        latte_image.set_size_request(300, 300);

        let latte_top_row = row();
        latte_top_row.set_halign(Align::Center);

        let latte_save_button = save_button("Save Latte");
        let latte_preview_button = preview_button();

        latte_column.append(&latte_top_row);
        latte_top_row.append(&latte_save_button);
        latte_top_row.append(&small_horizontal_spacer());
        latte_top_row.append(&latte_preview_button);
        latte_column.append(&small_vertical_spacer());
        latte_column.append(&latte_progressbar);
        latte_column.append(&latte_image);
        latte_column.set_visible(false);


        //Generated Frappe Column
        let frappe_column = accent_column();
        frappe_column.add_css_class("generated-column");
        frappe_column.set_margin_end(8);

        let frappe_progressbar = ProgressBar::new();
        frappe_progressbar.set_pulse_step(10000.0);
        frappe_progressbar.pulse();
        frappe_progressbar.set_visible(false);

        let frappe_image = Image::new();
        frappe_image.set_size_request(300, 300);

        let frappe_top_row = row();
        frappe_top_row.set_halign(Align::Center);

        let frappe_save_button = save_button("Save Frappe");
        let frappe_preview_button = preview_button();

        frappe_column.append(&frappe_top_row);
        frappe_top_row.append(&frappe_save_button);
        frappe_top_row.append(&small_horizontal_spacer());
        frappe_top_row.append(&frappe_preview_button);
        frappe_column.append(&small_vertical_spacer());
        frappe_column.append(&frappe_progressbar);
        frappe_column.append(&frappe_image);
        frappe_column.set_visible(false);

        //Generated Macchiato Column
        let macchiato_column = accent_column();
        macchiato_column.add_css_class("generated-column");
        macchiato_column.set_margin_bottom(8);
        macchiato_column.set_margin_end(8);

        let macchiato_progressbar = ProgressBar::new();
        macchiato_progressbar.set_pulse_step(10000.0);
        macchiato_progressbar.pulse();
        macchiato_progressbar.set_visible(false);

        let macchiato_image = Image::new();
        macchiato_image.set_size_request(300, 300);

        let macchiato_top_row = row();
        macchiato_top_row.set_halign(Align::Center);

        let macchiato_save_button = save_button("Save Macchiato");
        let macchiato_preview_button = preview_button();

        macchiato_column.append(&macchiato_top_row);
        macchiato_top_row.append(&macchiato_save_button);
        macchiato_top_row.append(&small_horizontal_spacer());
        macchiato_top_row.append(&macchiato_preview_button);
        macchiato_column.append(&small_vertical_spacer());
        macchiato_column.append(&macchiato_progressbar);
        macchiato_column.append(&macchiato_image);
        macchiato_column.set_visible(false);

        //Generated Mocha Column
        let mocha_column = accent_column();
        mocha_column.add_css_class("generated-column");
        mocha_column.set_margin_end(8);

        let mocha_progressbar = ProgressBar::new();
        mocha_progressbar.set_pulse_step(10000.0);
        mocha_progressbar.pulse();
        mocha_progressbar.set_visible(false);

        let mocha_image = Image::new();
        mocha_image.set_size_request(300, 300);

        let mocha_top_row = row();
        mocha_top_row.set_halign(Align::Center);

        let mocha_save_button = save_button("Save Mocha");
        let mocha_preview_button = preview_button();

        mocha_column.append(&mocha_top_row);
        mocha_top_row.append(&mocha_save_button);
        mocha_top_row.append(&small_horizontal_spacer());
        mocha_top_row.append(&mocha_preview_button);
        mocha_column.append(&small_vertical_spacer());
        mocha_column.append(&mocha_progressbar);
        mocha_column.append(&mocha_image);
        mocha_column.set_visible(false);

        //Generated Oled Column
        let oled_column = accent_column();
        oled_column.add_css_class("generated-column");
        oled_column.set_margin_bottom(8);
        oled_column.set_margin_end(8);

        let oled_progressbar = ProgressBar::new();
        oled_progressbar.set_pulse_step(10000.0);
        oled_progressbar.pulse();
        oled_progressbar.set_visible(false);

        let oled_image = Image::new();
        oled_image.set_size_request(300, 300);

        let oled_top_row = row();
        oled_top_row.set_halign(Align::Center);

        let oled_save_button = save_button("Save Oled");
        let oled_preview_button = preview_button();

        oled_column.append(&oled_top_row);
        oled_top_row.append(&oled_save_button);
        oled_top_row.append(&small_horizontal_spacer());
        oled_top_row.append(&oled_preview_button);
        oled_column.append(&small_vertical_spacer());
        oled_column.append(&oled_progressbar);
        oled_column.append(&oled_image);
        oled_column.set_visible(false);


        let results_grid = Grid::new();
        results_grid.attach(&latte_column, 0, 0, 1, 1);
        results_grid.attach(&frappe_column, 0, 1, 1, 1);
        results_grid.attach(&macchiato_column, 1, 0, 1, 1);
        results_grid.attach(&mocha_column, 1, 1, 1, 1);
        results_grid.attach(&oled_column, 2, 0, 1, 1);


        results_column.append(&image_results_column);

        let content_grid = Grid::new();


        content_grid.attach(&properties_column, 0, 0, 1, 1);
        content_grid.attach(&results_column, 1, 0, 5, 1);

        app_column.append(&content_scroll_box);
        content_scroll_box.set_child(Some(&content_grid));


        properties_column.append(&title("Properties"));
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&label("Noise level"));
        properties_column.append(&noise_scale_row);
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&select_image_button);
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&selected_image);
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&generate_images_button);
        results_column.append(&results_title);
        results_column.append(&results_grid);


        //Senders and Receivers
        let (generate_image_sender, generate_image_receiver) = MainContext::channel::<&str>(PRIORITY_DEFAULT);
        let (block_buttons_sender, block_buttons_receiver) = MainContext::channel::<bool>(PRIORITY_DEFAULT);


        let app_window = ApplicationWindow::builder()
            .application(app)
            .title("Catppuccinifier")
            .content(&app_column)
            .icon_name("catppuccinifier")
            .default_height(820)
            .default_width(1200)
            .build();


        app_window.show();

        //Functions
        let about_item_click = GestureClick::new();

        about_item_click.connect_released(clone!(
            @weak app,
            @weak app_window,
            @weak popover =>
            move |gesture, _, _, _|{

                gesture.set_state(gtk4::EventSequenceState::Claimed);

                popover.hide();
                popover.grab_focus();


                let about_window_column = column();
                about_window_column.append(&HeaderBar::new());

                let about_content_column = column();
                about_content_column.add_css_class("medium-padding");
                about_content_column.style_context().add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
                about_content_column.set_halign(Align::Center);


                let app_image = Image::new();
                let pixbuf = Pixbuf::from_file_at_size("/usr/share/pixmaps/catppuccinifier.png", 150, 150).unwrap();
                app_image.set_from_pixbuf(Some(&pixbuf));
                app_image.set_size_request(150, 150);


                let catppuccinifier_title = title("Catppuccinifier");
                catppuccinifier_title.set_halign(Align::Center);

                let lighttigerxiv = label("@lighttigerXIV");
                lighttigerxiv.set_halign(Align::Center);

                let version = label("2.0");
                version.add_css_class("version");
                version.style_context().add_provider(&app_css() ,STYLE_PROVIDER_PRIORITY_APPLICATION);
                version.set_halign(Align::Center);

                let source_code_column = row();
                source_code_column.add_css_class("about-item");
                source_code_column.style_context().add_provider(&app_css() ,STYLE_PROVIDER_PRIORITY_APPLICATION);

                let source_code = &label("Source Code");
                source_code.set_halign(Align::Start);
                source_code.set_hexpand(true);

                let open_icon = Image::builder().icon_name("web-browser-symbolic").build();

                source_code_column.append(source_code);
                source_code_column.append(&open_icon);



                about_window_column.append(&about_content_column);
                about_content_column.append(&app_image);
                about_content_column.append(&small_vertical_spacer());
                about_content_column.append(&catppuccinifier_title);
                about_content_column.append(&lighttigerxiv);
                about_content_column.append(&small_vertical_spacer());
                about_content_column.append(&version);
                about_content_column.append(&small_vertical_spacer());
                about_content_column.append(&source_code_column);


                let about_window = ApplicationWindow::builder()
                    .application(&app)
                    .title("About")
                    .default_width(300)
                    .default_height(-1)
                    .resizable(false)
                    .modal(true)
                    .content(&about_window_column)
                    .build();


                about_window.set_transient_for(Some(&app_window));
                about_window.show();


                let source_code_click = GestureClick::new();

                source_code_click.connect_released(|gesture, _, _, _|{
                    gesture.set_state(gtk4::EventSequenceState::Claimed);

                    if webbrowser::open("https://github.com/lighttigerXIV/catppuccinifier").is_ok(){
                        //Do Nothing
                    }
                });

                source_code_column.add_controller(source_code_click);
            }
        ));

        about_item.add_controller(about_item_click);

        noise_scale.connect_value_changed(clone!( @weak noise_level_label => move |scale|{

            noise_level_label.set_label((scale.value() as i32).to_string().as_str())

        }));

        let save_image = clone!(
            @weak app_window,
            @weak selected_image_name
            => move |flavor: &str| {

                let flv = flavor.to_string();

                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&app_window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = format!("{}-{}", flv, selected_image_name.borrow());

                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( move |dialog, response|{

                    if response == ResponseType::Ok{

                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){

                                let command = format!("cp /tmp/catppuccinifier/{}.png '{}'", flv , file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
        });

        latte_save_button.connect_clicked(clone!( @strong save_image => move |_| {
            save_image("latte");
        }));

        frappe_save_button.connect_clicked(clone!( @strong save_image => move |_| {
            save_image("frappe");
        }));

        macchiato_save_button.connect_clicked(clone!( @strong save_image => move |_| {
            save_image("macchiato");
        }));

        mocha_save_button.connect_clicked(clone!( @strong save_image => move |_| {
            save_image("mocha");
        }));

        oled_save_button.connect_clicked(clone!( @strong save_image => move |_| {
            save_image("oled");
        }));

        let preview_image = clone!(
            @weak app,
            @weak app_window
            => move |flavor: &str|{

                let preview_column = column();
                preview_column.append(&HeaderBar::new());

                let image_pixbuf = Pixbuf::from_file(format!("/tmp/catppuccinifier/{}.png", flavor)).unwrap();
                
                let image = Image::new();
                image.set_from_pixbuf(Some(&image_pixbuf));
                image.set_vexpand(true);


                preview_column.append(&image);

                let preview_window = ApplicationWindow::builder()
                    .application(&app)
                    .title(format!("{} image", &flavor))
                    .resizable(false)
                    .default_height(700)
                    .default_width(700)
                    .modal(true)
                    .content(&preview_column)
                    .build();


                preview_window.set_transient_for(Some(&app_window));
                preview_window.show();
            }
        );

        latte_preview_button.connect_clicked(clone!(@strong preview_image => move |_|{
            preview_image("latte");
        }));

        frappe_preview_button.connect_clicked(clone!(@strong preview_image => move |_|{
            preview_image("frappe");
        }));

        macchiato_preview_button.connect_clicked(clone!(@strong preview_image => move |_|{
            preview_image("macchiato");
        }));

        mocha_preview_button.connect_clicked(clone!(@strong preview_image => move |_|{
            preview_image("mocha");
        }));

        oled_preview_button.connect_clicked(clone!(@strong preview_image => move |_|{
            preview_image("oled");
        }));


        generate_images_button.connect_clicked(clone!(
            @weak noise_scale,
            @weak selected_image_path,
            @weak select_image_button,
            @weak generate_images_button,
            @weak latte_column,
            @weak latte_image,
            @weak latte_save_button,
            @weak latte_preview_button,
            @weak latte_progressbar,
            @weak frappe_column,
            @weak frappe_image,
            @weak frappe_save_button,
            @weak frappe_preview_button,
            @weak frappe_progressbar,
            @weak macchiato_column,
            @weak macchiato_image,
            @weak macchiato_save_button,
            @weak macchiato_preview_button,
            @weak macchiato_progressbar,
            @weak mocha_column,
            @weak mocha_image,
            @weak mocha_save_button,
            @weak mocha_preview_button,
            @weak mocha_progressbar,
            @weak oled_column,
            @weak oled_image,
            @weak oled_save_button,
            @weak oled_preview_button,
            @weak oled_progressbar=>
            move |_|{

                let noise_level = noise_scale.value() as i32;
                let image_path = selected_image_path.borrow().to_string();

                let generate_sender = generate_image_sender.clone();
                let block_sender = block_buttons_sender.clone();

                block_sender.send(true).expect("");

                latte_column.set_visible(true);
                latte_image.clear();
                latte_progressbar.set_visible(true);

                frappe_column.set_visible(true);
                frappe_image.clear();
                frappe_progressbar.set_visible(true);

                macchiato_column.set_visible(true);
                macchiato_image.clear();
                macchiato_progressbar.set_visible(true);

                mocha_column.set_visible(true);
                mocha_image.clear();
                mocha_progressbar.set_visible(true);

                oled_column.set_visible(true);
                oled_image.clear();
                oled_progressbar.set_visible(true);


                thread::spawn(move || {

                    //Creates the temp folder
                    Command::new("/bin/sh")
                        .arg("-c")
                        .arg("mkdir -p /tmp/catppuccinifier/")
                        .output()
                        .expect("Error while converting images");

                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/latte.png -hald-clut /tmp/catppuccinifier/latte.png", image_path, noise_level);


                    //Generates Latte Image
                    Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                    generate_sender.send("latte").expect("");


                    //Generates Frappe Image
                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/frappe.png -hald-clut /tmp/catppuccinifier/frappe.png", image_path, noise_level);

                    Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");


                    generate_sender.send("frappe").expect("");


                    //Generates Macchiato Image
                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/macchiato.png -hald-clut /tmp/catppuccinifier/macchiato.png", image_path, noise_level);

                    Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");


                    generate_sender.send("macchiato").expect("");


                    //Generates Mocha Image
                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/mocha.png -hald-clut /tmp/catppuccinifier/mocha.png", image_path, noise_level);

                    Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");


                    generate_sender.send("mocha").expect("");

                    //Generates Oled Image
                    let command = format!("convert '{}' $HOME/.local/share/catppuccinifier/flavors/noise-{}/oled.png -hald-clut /tmp/catppuccinifier/oled.png", image_path, noise_level);

                    Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");


                    generate_sender.send("oled").expect("");

                    block_sender.send(false).expect("")
                });
            }
        ));

        generate_image_receiver.attach(
            None,
            clone!(
                @weak latte_image,
                @weak latte_progressbar,
                @weak frappe_image,
                @weak frappe_progressbar,
                @weak macchiato_image,
                @weak macchiato_progressbar,
                @weak mocha_image,
                @weak mocha_progressbar,
                @weak oled_image,
                @weak oled_progressbar=>
                @default-return Continue(false), move |flavor|{

                    if flavor == "latte"{

                        let latte_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/latte.png")
                            .expect("Failed to load latte image ...");

                        latte_image.set_from_pixbuf(Some(&latte_pixbuf));

                        latte_progressbar.set_visible(false);
                    }

                    if flavor == "frappe"{

                        let frappe_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/frappe.png")
                            .expect("Failed to load frappe image ...");

                        frappe_image.set_from_pixbuf(Some(&frappe_pixbuf));

                        frappe_progressbar.set_visible(false);
                    }

                    if flavor == "macchiato"{

                        let macchiato_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/macchiato.png")
                            .expect("Failed to load macchiato image ...");

                        macchiato_image.set_from_pixbuf(Some(&macchiato_pixbuf));

                        macchiato_progressbar.set_visible(false);
                    }

                    if flavor == "mocha"{

                        let mocha_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/mocha.png")
                            .expect("Failed to load mocha image ...");

                        mocha_image.set_from_pixbuf(Some(&mocha_pixbuf));

                        mocha_progressbar.set_visible(false);
                    }

                    if flavor == "oled"{

                        let oled_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/oled.png")
                            .expect("Failed to load oled image ...");

                        oled_image.set_from_pixbuf(Some(&oled_pixbuf));

                        oled_progressbar.set_visible(false);
                    }

                    Continue(true)
            }),
        );

        block_buttons_receiver.attach(
            None,
            clone!(
                @weak select_image_button,
                @weak generate_images_button,
                @weak latte_save_button,
                @weak latte_preview_button,
                @weak frappe_save_button,
                @weak frappe_preview_button,
                @weak macchiato_save_button,
                @weak macchiato_preview_button,
                @weak mocha_save_button,
                @weak mocha_preview_button,
                @weak oled_save_button,
                @weak oled_preview_button=>
                @default-return Continue(false), move |block|{

                    select_image_button.set_sensitive(!block);
                    generate_images_button.set_sensitive(!block);
                    latte_save_button.set_sensitive(!block);
                    latte_preview_button.set_sensitive(!block);
                    frappe_save_button.set_sensitive(!block);
                    frappe_preview_button.set_sensitive(!block);
                    macchiato_save_button.set_sensitive(!block);
                    macchiato_preview_button.set_sensitive(!block);
                    mocha_save_button.set_sensitive(!block);
                    mocha_preview_button.set_sensitive(!block);
                    oled_save_button.set_sensitive(!block);
                    oled_preview_button.set_sensitive(!block);

                    Continue(true)
            }),
        );

        select_image_button.connect_clicked(move |_| {
            let file_chooser = FileChooserDialog::new(
                Some("Select Image"),
                Some(&app_window),
                FileChooserAction::Open,
                &[
                    ("Select", ResponseType::Ok),
                    ("Cancel", ResponseType::Cancel),
                ],
            );


            let files_filter = FileFilter::new();
            files_filter.set_name(Some("Image Files"));
            files_filter.add_mime_type("image/png");
            files_filter.add_mime_type("image/jpeg");
            files_filter.add_mime_type("image/jpg");
            files_filter.add_mime_type("image/gif");
            files_filter.add_mime_type("image/webp");

            file_chooser.set_filter(&files_filter);


            file_chooser.connect_response(clone!(
                @weak selected_image_path,
                @weak selected_image_name,
                @weak selected_image,
                @weak generate_images_button,
                @weak latte_column,
                @weak frappe_column,
                @weak macchiato_column,
                @weak mocha_column,
                @weak oled_column
                => move |dialog, response| {

                    if response == ResponseType::Ok {

                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){

                                selected_image_path.replace(file.path().unwrap().to_string_lossy().to_string());
                                selected_image_name.replace(file.path().unwrap().file_name().unwrap().to_str().unwrap().to_string());

                                let image_pixbuf = Pixbuf::from_file(file.path().unwrap().to_string_lossy().to_string().as_str())
                                    .expect("Failed to load selected image ...");


                                selected_image.set_from_pixbuf(Some(&image_pixbuf));

                                selected_image.set_visible(true);

                                generate_images_button.set_visible(true);


                                latte_column.set_visible(false);
                                frappe_column.set_visible(false);
                                macchiato_column.set_visible(false);
                                mocha_column.set_visible(false);
                                oled_column.set_visible(false);
                            }
                        }
                    }

                    dialog.emit_close();
            }));


            file_chooser.show();
        }
        );
    });

    app.run();
}
