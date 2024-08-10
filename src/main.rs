use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

const APP_ID: &str = "org.gtk_rs.HelloWorld3";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press Me!")
        .height_request(60)
        .width_request(60)
        .margin(20)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        if button.label().as_deref() == Some("Hello World!"){
        button.set_label("Press Me!");
        }else {
            // Set the label to "Hello World!" after the button has been clicked on
            button.set_label("Hello World!");
        }
    });

    // Create a window with a default size
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(300) // Set a default width
        .default_height(200) // Set a default height
        .child(&button)
        .build();
    button.show();
    // Present window
    window.present();
}