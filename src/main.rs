use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Button};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.winfy")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Exit")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    button.connect_clicked(move |button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");

    });

    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("WinFY")
        .child(&button)
        .build();

    // Present window
    window.present();
}
