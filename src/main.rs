use gtk4::prelude::*;
use gtk4::{Align, Application, ApplicationWindow, Button};
use gtk4::glib::clone;

fn main() {
    let application = Application::new(Some("com.winjoexd.winfy"), Default::default());
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

    window.set_child(Some(&button));

    window.show();
}
