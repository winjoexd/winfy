use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(Some("org.winjoexd.winfy"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title(Some("WinFY"));
    window.set_default_size(640, 480);

    let button = Button::builder()
        .label("Exit")
        .halign(Align::End)
        .valign(Align::End)
        .margin_bottom(12)
        .margin_end(12)
        .build();

    button.connect_clicked(clone!(@strong window => move |_| {
        window.close();
    }));

    let scroll_window = gtk4::ScrolledWindow::builder()
        .height_request(240)
        .width_request(310)
        .build();

    let text_input = gtk4::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    scroll_window.set_child(Some(&text_input));

    let scroll_window_2 = gtk4::ScrolledWindow::builder()
        .height_request(240)
        .width_request(310)
        .build();

    let text_output = gtk4::TextView::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

    scroll_window_2.set_child(Some(&text_output));

    let container_texts = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Horizontal)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .spacing(8)
        .build();
    container_texts.append(&scroll_window);
    container_texts.append(&scroll_window_2);

    let container = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(8)
        .margin_bottom(8)
        .margin_start(8)
        .margin_end(8)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .spacing(8)
        .build();

    container.append(&container_texts);
    container.append(&button);
    window.set_child(Some(&container));

    window.show();
}
