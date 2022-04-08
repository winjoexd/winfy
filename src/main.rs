use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Button};
use gtk4::glib::clone;

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
        .vexpand(true)
        .hexpand(true)
        .build();

    let text_input = gtk4::TextView::builder()
        .build();
    
    scroll_window.set_child(Some(&text_input));

    let container = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .spacing(24)
        .build();

    container.append(&scroll_window);
    container.append(&button);
    window.set_child(Some(&container));

    window.show();
}
