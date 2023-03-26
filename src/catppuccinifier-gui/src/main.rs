use adw::{prelude::*, Application, ApplicationWindow, HeaderBar};
use gtk4::{
    Align, Box, Button, CssProvider, FileChooserAction, FileChooserDialog, FileFilter, Grid, Image,
    Label, Orientation, ResponseType, Scale, STYLE_PROVIDER_PRIORITY_APPLICATION, GestureClick, MenuButton, Popover,
};

use gdk_pixbuf::Pixbuf;
use glib::{clone, MainContext, PRIORITY_DEFAULT};
use std::cell::RefCell;
use std::rc::Rc;
use std::process::Command;
use std::thread;
use webbrowser;

fn get_css() -> CssProvider {
    let css_provider = CssProvider::new();
    css_provider.load_from_data(
        "

        .title {
            font-size: 1.2rem;
        }

        .accent-column {
            background: @view_bg_color;
            border-radius: 8px;
            padding: 8px;
        }

        .button {
            border-radius: 1rem;
        }

        .clickable:hover {
            background: @accent_bg_color;
            border-radius: 8px;
        }   
    ",
    );

    css_provider
}

fn small_vertical_spacer() -> Box {
    let small_box = Box::new(Orientation::Vertical, 0);
    small_box.set_size_request(1, 8);

    small_box
}

fn column() -> Box {
    let column = Box::new(Orientation::Vertical, 0);

    column
}

fn row() -> Box {
    let column = Box::new(Orientation::Horizontal, 0);

    column
}

fn title(text: &str) -> Label {
    let title = Label::new(Some(text));
    title.set_hexpand(true);
    title.add_css_class("title");
    title
        .style_context()
        .add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

    title
}

fn accent_column() -> Box {
    let column = Box::new(Orientation::Vertical, 0);
    column.add_css_class("accent-column");
    column
        .style_context()
        .add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

    column
}

#[tokio::main]
async fn main() {

    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();



    application.connect_activate(|app| {
        let main_colum = column();
        let header_bar = HeaderBar::new();
        

        let menu_button = MenuButton::new();
        
        menu_button.set_icon_name("open-menu-symbolic");

        let popover = Popover::new();

        let popover_content = column();

        let about_item = Label::new(Some("About"));
        about_item.add_css_class("clickable");
        about_item.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);  

        popover_content.append(&about_item);      
        popover.set_autohide(true);

        popover.set_child(Some(&popover_content));
        menu_button.set_popover(Some(&popover));
        

        header_bar.pack_end(&menu_button);
        main_colum.append(&header_bar);



        let properties_column = column();
        properties_column.set_hexpand(true);
        properties_column.set_vexpand(true);

        let results_column = column();
        results_column.set_margin_start(8);
        results_column.set_hexpand(true);
        results_column.set_vexpand(true);
        

        let content_grid = Grid::new();
        content_grid.set_margin_top(10);
        content_grid.set_margin_bottom(10);
        content_grid.set_margin_start(10);
        content_grid.set_margin_end(10);

        content_grid.attach(&properties_column, 0, 0, 4, 1);
        content_grid.attach(&results_column, 5, 0, 6, 1);
        


        //-----------------------------------
        // Properties Column
        //-----------------------------------

        let properties_title = title("Properties");
        properties_title.set_halign(Align::Start);

        let noise_level_column = accent_column();
        let noise_level_label = Label::new(Some("Noise level"));
        noise_level_label.set_halign(Align::Start);

        let noise_scale_row = row();

        let noise_scale = Scale::with_range(Orientation::Horizontal, 0.0, 4.0, 1.0);
        noise_scale.set_hexpand(true);
        noise_scale.set_digits(0);

        let noise_scale_label = Label::new(Some("0"));

        let select_image_column = accent_column();

        let select_image_label = Label::new(Some("Select Image"));
        select_image_label.set_halign(Align::Start);

        let select_image_button = Button::with_label("Select Image");
        select_image_button.add_css_class("button");
        select_image_button
            .style_context()
            .add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

        let empty_string = "";
        let selected_image_path = Rc::new(RefCell::new(empty_string.to_string()));
        let selected_image_name = Rc::new(RefCell::new(empty_string.to_string()));
        let selected_image = Image::new();

        let generate_images_button = Button::with_label("Generate Images");
        generate_images_button.add_css_class("button");
        generate_images_button.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);


        noise_scale.connect_value_changed(clone!( @strong noise_scale_label => move |scale|{

            noise_scale_label.set_label((scale.value() as i32).to_string().as_str())
        }));

    

        properties_column.append(&properties_title);
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&noise_level_column);
        noise_level_column.append(&noise_level_label);
        noise_level_column.append(&noise_scale_row);
        noise_scale_row.append(&noise_scale);
        noise_scale_row.append(&noise_scale_label);
        properties_column.append(&small_vertical_spacer());
        properties_column.append(&select_image_column);
        select_image_column.append(&select_image_label);
        select_image_column.append(&small_vertical_spacer());
        select_image_column.append(&select_image_button);
        

        //-----------------------------------
        // Results Column
        //-----------------------------------

        

        let results_title = title("Results");
        results_title.set_halign(Align::Start);
        
        let results_grid = Grid::new();

        let generated_latte_column = accent_column();
        let latte_label = Label::new(Some("Latte"));
        latte_label.set_halign(Align::Start);
        let latte_image = Image::new();
        latte_image.set_size_request(350, 350);
        generated_latte_column.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
        generated_latte_column.set_margin_top(8);
        generated_latte_column.set_margin_bottom(8);
        generated_latte_column.set_margin_start(8);
        generated_latte_column.set_margin_end(8);


        let generated_frappe_column = accent_column();
        let frappe_label = Label::new(Some("Frappe"));
        frappe_label.set_halign(Align::Start);
        let frappe_image = Image::new();
        frappe_image.set_size_request(350, 350);
        generated_frappe_column.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
        generated_frappe_column.set_margin_top(8);
        generated_frappe_column.set_margin_bottom(8);
        generated_frappe_column.set_margin_start(8);
        generated_frappe_column.set_margin_end(8);

        let generated_macchiato_column = accent_column();
        let macchiato_label = Label::new(Some("Macchiato"));
        macchiato_label.set_halign(Align::Start);
        let macchiato_image = Image::new();
        macchiato_image.set_size_request(350, 350);
        generated_macchiato_column.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
        generated_macchiato_column.set_margin_top(8);
        generated_macchiato_column.set_margin_bottom(8);
        generated_macchiato_column.set_margin_start(8);
        generated_macchiato_column.set_margin_end(8);

        let generated_mocha_column = accent_column();
        let mocha_label = Label::new(Some("Mocha"));
        mocha_label.set_halign(Align::Start);
        let mocha_image = Image::new();
        mocha_image.set_size_request(350, 350);
        generated_mocha_column.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
        generated_mocha_column.set_margin_top(8);
        generated_mocha_column.set_margin_bottom(8);
        generated_mocha_column.set_margin_start(8);
        generated_mocha_column.set_margin_end(8);

        let generated_oled_column = accent_column();
        let oled_label = Label::new(Some("Oled"));
        oled_label.set_halign(Align::Start);
        let oled_image = Image::new();
        oled_image.set_size_request(350, 350);
        generated_oled_column.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);
        generated_oled_column.set_margin_top(8);
        generated_oled_column.set_margin_bottom(8);
        generated_oled_column.set_margin_start(8);
        generated_oled_column.set_margin_end(8);

        results_column.append(&results_title);

        generated_latte_column.append(&latte_label);
        generated_latte_column.append(&latte_image);

        generated_frappe_column.append(&frappe_label);
        generated_frappe_column.append(&frappe_image);

        generated_macchiato_column.append(&macchiato_label);
        generated_macchiato_column.append(&macchiato_image);

        generated_mocha_column.append(&mocha_label);
        generated_mocha_column.append(&mocha_image);

        generated_oled_column.append(&oled_label);
        generated_oled_column.append(&oled_image);

        results_grid.attach(&generated_latte_column, 0, 0, 1, 1);
        results_grid.attach(&generated_frappe_column, 1, 0, 1, 1);
        results_grid.attach(&generated_macchiato_column, 0, 1, 1, 1);
        results_grid.attach(&generated_mocha_column, 1, 1, 1, 1);
        results_grid.attach(&generated_oled_column, 2, 0, 1, 1);

        results_column.append(&results_grid);


        let window = ApplicationWindow::builder()
            .application(app)
            .title("Catppuccinifier")
            .content(&main_colum)
            .build();

        

        window.set_icon_name(Some("catppuccinifier"));

        main_colum.append(&content_grid);

        window.show();


        //-------------------------------
        // Actions
        //-------------------------------

        let about_item_click = GestureClick::new();
        let generated_latte_click = GestureClick::new();
        let generated_frappe_click = GestureClick::new();
        let generated_macchiato_click = GestureClick::new();
        let generated_mocha_click = GestureClick::new();
        let generated_oled_click = GestureClick::new();


        about_item_click.connect_released( clone!( @strong app, @strong window, @strong popover => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);


            popover.grab_focus();

            let about_window_main_box = column();
            about_window_main_box.append(&HeaderBar::new());

            let about_column = column();
            about_column.set_margin_top(8);
            about_column.set_margin_bottom(8);
            about_column.set_margin_start(8);
            about_column.set_margin_end(8);
            about_window_main_box.append(&about_column);


            
            let version_title = title("Version");
            version_title.set_halign(Align::Start);

            let app_version = Label::new(Some("0.1-BETA"));
            app_version.set_halign(Align::Start);

            let source_title = title("Source");
            source_title.set_halign(Align::Start);

            let source_link = Label::new(Some("https://github.com/lighttigerXIV/catppuccinifier"));
            source_link.set_halign(Align::Start);

            let source_link_click = GestureClick::new();
            source_link_click.connect_released(move |gesture, _, _, _|{
                gesture.set_state(gtk4::EventSequenceState::Claimed);

                if webbrowser::open("https://github.com/lighttigerXIV/catppuccinifier").is_ok(){
                    //DO Nothing
                }
            });

            source_link.add_controller(source_link_click);
            
            
            about_column.append(&version_title);
            about_column.append(&app_version);
            about_column.append(&small_vertical_spacer());
            about_column.append(&source_title);
            about_column.append(&source_link);


            let about_window = ApplicationWindow::builder()
                .application(&app)
                .title("About")
                .content(&about_window_main_box)
                .default_width(400)
                .default_height(80)
                .build();


            about_window.set_resizable(false);
            
            //Disables main window until about it's closed
            about_window.set_transient_for(Some(&window));
            about_window.set_modal(true);

            about_window.show();

            popover.hide();
        }));

        about_item.add_controller(about_item_click);


        generated_latte_click.connect_released(clone!( @strong window, @strong generated_latte_column, @strong selected_image_name => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            if generated_latte_column.has_css_class("clickable"){

                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = String::from("latte-") + selected_image_name.borrow().to_string().as_str();
                
                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( |dialog, response|{

                    if response == ResponseType::Ok{


                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){
                                
                                let command = format!("cp /tmp/catppuccinifier/latte.png {}", file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving latte image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
            }
        }));

        generated_latte_column.add_controller(generated_latte_click);

        generated_frappe_click.connect_released(clone!( @strong window, @strong generated_frappe_column, @strong selected_image_name => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            if generated_frappe_column.has_css_class("clickable"){
                
                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = String::from("frappe-") + selected_image_name.borrow().to_string().as_str();
                
                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( |dialog, response|{

                    if response == ResponseType::Ok{


                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){
                                
                                let command = format!("cp /tmp/catppuccinifier/frappe.png {}", file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving frappe image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
            }
            
        }));

        generated_frappe_column.add_controller(generated_frappe_click);

        generated_macchiato_click.connect_released(clone!( @strong window, @strong generated_macchiato_column, @strong selected_image_name => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            if generated_macchiato_column.has_css_class("clickable"){

                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = String::from("macchiato-") + selected_image_name.borrow().to_string().as_str();
                
                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( |dialog, response|{

                    if response == ResponseType::Ok{


                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){
                                
                                let command = format!("cp /tmp/catppuccinifier/macchiato.png {}", file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving macchiato image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
            }
            
        }));

        generated_macchiato_column.add_controller(generated_macchiato_click);

        generated_mocha_click.connect_released(clone!( @strong window, @strong generated_mocha_column, @strong selected_image_name => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            if generated_mocha_column.has_css_class("clickable"){

                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = String::from("mocha-") + selected_image_name.borrow().to_string().as_str();
                
                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( |dialog, response|{

                    if response == ResponseType::Ok{


                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){
                                
                                let command = format!("cp /tmp/catppuccinifier/mocha.png {}", file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving mocha image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
            }
            
        }));

        generated_mocha_column.add_controller(generated_mocha_click);

        generated_oled_click.connect_released(clone!( @strong window, @strong generated_oled_column, @strong selected_image_name => move |gesture, _, _, _|{
            gesture.set_state(gtk4::EventSequenceState::Claimed);

            if generated_oled_column.has_css_class("clickable"){
            
                let file_saver = FileChooserDialog::new(
                    Some("Select Image"),
                    Some(&window),
                    FileChooserAction::Save,
                    &[
                        ("Save", ResponseType::Ok),
                        ("Cancel", ResponseType::Cancel),
                    ],
                );

                let default_name = String::from("oled-") + selected_image_name.borrow().to_string().as_str();
                
                file_saver.set_current_name(default_name.as_str());

                file_saver.connect_response( |dialog, response|{

                    if response == ResponseType::Ok{


                        if let Some(file) = dialog.file() {

                            if file.path().is_some(){
                                
                                let command = format!("cp /tmp/catppuccinifier/oled.png {}", file.path().unwrap().to_string_lossy());

                                Command::new("/bin/sh")
                                    .arg("-c")
                                    .arg(command)
                                    .output()
                                    .expect("Error saving oled image");
                            }
                        }
                    }

                    dialog.emit_close();
                });

                file_saver.show();
            }
        }));

        generated_oled_column.add_controller(generated_oled_click);

        

        let (block_buttons_sender, block_buttons_receiver) = MainContext::channel::<bool>(PRIORITY_DEFAULT);
        let (generate_images_sender, generate_images_receiver) = MainContext::channel::<bool>(PRIORITY_DEFAULT);

        block_buttons_receiver.attach(
            None,
            clone!(
                @strong select_image_button,
                @strong generate_images_button,
                @strong generated_latte_column,
                @strong generated_frappe_column,
                @strong generated_macchiato_column,
                @strong generated_mocha_column,
                @strong generated_oled_column,
                => @default-return Continue(false),
                            move |block_buttons| {
                                
                                if block_buttons{

                                    select_image_button.set_sensitive(false);
                                    generate_images_button.set_sensitive(false);
                                    generated_latte_column.remove_css_class("clickable");
                                    generated_frappe_column.remove_css_class("clickable");
                                    generated_macchiato_column.remove_css_class("clickable");
                                    generated_mocha_column.remove_css_class("clickable");
                                    generated_oled_column.remove_css_class("clickable");

                                } else {
                                
                                    select_image_button.set_sensitive(true);
                                    generate_images_button.set_sensitive(true);
                                    generated_latte_column.add_css_class("clickable");
                                    generated_frappe_column.add_css_class("clickable");
                                    generated_macchiato_column.add_css_class("clickable");
                                    generated_mocha_column.add_css_class("clickable");
                                    generated_oled_column.add_css_class("clickable");
                                }
                                

                                Continue(true)
                            }
                ),
        );

        generate_images_receiver.attach(
            None,
            clone!(
                @strong latte_image,
                @strong frappe_image,
                @strong macchiato_image,
                @strong mocha_image,
                @strong oled_image,
                => @default-return Continue(false),
                            move |_| {
                                
                                let latte_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/latte.png")
                                    .expect("Failed to load latte image ...");

                                latte_image.set_from_pixbuf(Some(&latte_pixbuf));

                                let frappe_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/frappe.png")
                                    .expect("Failed to load frappe image ...");

                                frappe_image.set_from_pixbuf(Some(&frappe_pixbuf));

                                let macchiato_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/macchiato.png")
                                    .expect("Failed to load macchiato image ...");

                                macchiato_image.set_from_pixbuf(Some(&macchiato_pixbuf));

                                let mocha_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/mocha.png")
                                    .expect("Failed to load mocha image ...");

                                mocha_image.set_from_pixbuf(Some(&mocha_pixbuf));

                                let oled_pixbuf = Pixbuf::from_file("/tmp/catppuccinifier/oled.png")
                                    .expect("Failed to load oled image ...");

                                oled_image.set_from_pixbuf(Some(&oled_pixbuf));

                                Continue(true)
                            }
                ), 
        );

        generate_images_button.connect_clicked( 
            clone!(
                @strong selected_image_path,
                @strong latte_image,
                @strong frappe_image,
                @strong macchiato_image,
                @strong mocha_image,
                @strong oled_image
                => move |_| {
            
                    let noise_level = noise_scale.value() as i32;
                    let image_path = selected_image_path.borrow().to_string();
                    
                    //Clears previous images
                    latte_image.clear();
                    frappe_image.clear();
                    macchiato_image.clear();
                    mocha_image.clear();
                    oled_image.clear();


                    thread::spawn( clone!(@strong generate_images_sender, @strong block_buttons_sender => move || {
                    
                        block_buttons_sender.send(true).expect("Error sending block buttons true");

                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg("mkdir -p /tmp/catppuccinifier/")
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert {} $HOME/.local/share/catppuccinifier/flavors/noise-{}/latte.png -hald-clut /tmp/catppuccinifier/latte.png", image_path, noise_level);

                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert {} $HOME/.local/share/catppuccinifier/flavors/noise-{}/frappe.png -hald-clut /tmp/catppuccinifier/frappe.png", image_path, noise_level);
                        
                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert {} $HOME/.local/share/catppuccinifier/flavors/noise-{}/macchiato.png -hald-clut /tmp/catppuccinifier/macchiato.png", image_path, noise_level);
                        
                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert {} $HOME/.local/share/catppuccinifier/flavors/noise-{}/mocha.png -hald-clut /tmp/catppuccinifier/mocha.png", image_path, noise_level);
                        
                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                        let command = format!("convert {} $HOME/.local/share/catppuccinifier/flavors/noise-{}/oled.png -hald-clut /tmp/catppuccinifier/oled.png", image_path, noise_level);
                        
                        Command::new("/bin/sh")
                            .arg("-c")
                            .arg(&command)
                            .output()
                            .expect("Error while converting images");

                        generate_images_sender.send(true).expect("Error sending generate images");

                        block_buttons_sender.send(false).expect("Error sending block buttons false");
                    }));
                }   
            )
        );

        select_image_button.connect_clicked( clone!( @strong window => move |_| {

            let file_chooser = FileChooserDialog::new(
                Some("Select Image"),
                Some(&window),
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


            file_chooser.connect_response( 
                clone!(
                    @strong properties_column,
                    @strong selected_image_path,
                    @strong selected_image,
                    @strong selected_image_name,
                    @strong select_image_column,
                    @strong file_chooser,
                    @strong generate_images_button,
                    @strong latte_image,
                    @strong generated_latte_column,
                    @strong frappe_image,
                    @strong generated_frappe_column,
                    @strong macchiato_image,
                    @strong generated_macchiato_column,
                    @strong mocha_image,
                    @strong generated_mocha_column,
                    @strong oled_image,
                    @strong generated_oled_column,
                    => move |dialog, response| {

                if response == ResponseType::Ok {

                    if let Some(file) = dialog.file() {

                        if file.path().is_some(){

                            latte_image.clear();
                            frappe_image.clear();
                            macchiato_image.clear();
                            mocha_image.clear();
                            oled_image.clear();

                            generated_latte_column.remove_css_class("clickable");
                            generated_frappe_column.remove_css_class("clickable");
                            generated_macchiato_column.remove_css_class("clickable");
                            generated_mocha_column.remove_css_class("clickable");
                            generated_oled_column.remove_css_class("clickable");


                            selected_image_path.replace(file.path().unwrap().to_string_lossy().to_string());
                            selected_image_name.replace(file.path().unwrap().file_name().unwrap().to_str().unwrap().to_string());

                            let image_pixbuf = Pixbuf::from_file(file.path().unwrap().to_string_lossy().to_string().as_str())
                                .expect("Failed to load selected image ...");

                            selected_image.set_from_pixbuf(Some(&image_pixbuf));
                            selected_image.set_size_request(-1, 400);
                            selected_image.style_context().add_provider(&get_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

                            select_image_column.append(&small_vertical_spacer());
                            select_image_column.append(&selected_image);

                            select_image_column.append(&small_vertical_spacer());
                            select_image_column.append(&generate_images_button);
                        }
                    }
                }

                dialog.emit_close();
            }));

            file_chooser.show();
        }));
    });

    application.run();
}
