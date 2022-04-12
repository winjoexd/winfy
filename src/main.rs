use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Button, ScrolledWindow, TextView, Box, Orientation, DropDown};
mod fy;
use crate::fy::fy::fy_handle;

fn main() {
    let application = Application::new(Some("org.winjoexd.winfy"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("WinFY"));
    window.set_default_size(640, 300);

    let button_exit = Button::builder()
        .label("Exit")
         .build();

    button_exit.connect_clicked(clone!(@strong window => move |_| {
        window.close();
    }));

    let button_fy = Button::builder()
         .label("FY")
        .build();

    let container_buttons = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::End)
        .valign(Align::Center)
        .margin_end(8)
        .spacing(100)
        .build();
    container_buttons.append(&button_fy);
    container_buttons.append(&button_exit);

    let langs_items = ["ENG", "CHT"];
    
    let langs_input = DropDown::from_strings(&langs_items);
    let langs_output = DropDown::from_strings(&langs_items);
    
    let container_langs = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(8)
        .build();
    container_langs.append(&langs_input);
    container_langs.append(&langs_output);
    langs_output.set_selected(1);

    let scroll_window_1 = ScrolledWindow::builder()
        .height_request(240)
        .width_request(310)
        .build();

    let text_input = TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    scroll_window_1.set_child(Some(&text_input));

    let scroll_window_2 = ScrolledWindow::builder()
        .height_request(240)
        .width_request(310)
        .build();

    let text_output = TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .editable(false)
        .build();

    scroll_window_2.set_child(Some(&text_output));

    let container_texts = Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .spacing(8)
        .build();
    container_texts.append(&scroll_window_1);
    container_texts.append(&scroll_window_2);

    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(8)
        .build();

    container.append(&container_langs);
    container.append(&container_texts);
    container.append(&container_buttons);
    window.set_child(Some(&container));

    button_fy.connect_clicked(clone!(@strong text_output => move |_| {
        let start = text_input.buffer().start_iter();
        let end = text_input.buffer().end_iter();
        let read_str = text_input.buffer().text(&start, &end, false);
        println!("{}", read_str);
        let output_str = fy_handle(read_str.to_string());
        text_output.buffer().set_text(&format!("{}", output_str).to_string());
    }));

    window.show();
}
