use gdk_pixbuf::Pixbuf;
use gtk4::{prelude::*, Box, Orientation, Label, STYLE_PROVIDER_PRIORITY_APPLICATION, Align, Button, Image, STYLE_PROVIDER_PRIORITY_USER, DrawingArea, IconTheme};
use crate::css::app_css;


pub fn column() -> Box {
    let column = Box::new(Orientation::Vertical, 0);

    column
}

pub fn row() -> Box {
    let row = Box::new(Orientation::Horizontal, 0);

    row
}

pub fn accent_column() -> Box {
    let column = Box::new(Orientation::Vertical, 0);
    column.add_css_class("foreground");
    column.style_context()
        .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

    column
}


pub fn title(text: &str) -> Label {
    let title = Label::new(Some(text));

    title.add_css_class("title");

    title.set_halign(Align::Start);

    title.style_context()
        .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

    title
}

pub fn label(text: &str) -> Label {
    let label = Label::new(Some(text));

    label.set_halign(Align::Start);

    label.style_context()
        .add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_APPLICATION);

    label
}

pub fn small_vertical_spacer() -> Box {
    let spacer = Box::new(Orientation::Vertical, 0);

    spacer.set_height_request(8);

    spacer
}

pub fn small_horizontal_spacer() -> Box {
    let spacer = Box::new(Orientation::Vertical, 0);

    spacer.set_width_request(8);

    spacer
}

pub fn save_button(text: &str) -> Button {
    let button = Button::new();
    button.add_css_class("round-button");
    button.style_context().add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_USER);
    let row = Box::new(Orientation::Horizontal, 0);

    let label = Label::new(Some(text));
    let icon = Image::from_icon_name("ymuse-save-symbolic");

    icon.set_height_request(button.height());

    row.append(&label);
    row.append(&small_horizontal_spacer());
    row.append(&icon);

    button.set_child(Some(&row));

    button
}

pub fn preview_button() -> Button {
    let button = Button::new();
    button.add_css_class("round-button");
    button.style_context().add_provider(&app_css(), STYLE_PROVIDER_PRIORITY_USER);
    let row = Box::new(Orientation::Horizontal, 0);

    let label = Label::new(Some("Preview"));
    let icon = Image::from_icon_name("system-search-symbolic");

    icon.set_height_request(button.height());

    row.append(&label);
    row.append(&small_horizontal_spacer());
    row.append(&icon);

    button.set_child(Some(&row));

    button
}