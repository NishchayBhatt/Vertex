
use gtk::{prelude::*, Label};
use gtk::{glib, Application, ApplicationWindow, Button,Box as GtkBox,Orientation};
use std::rc::Rc;
use std::cell::RefCell;


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
    let number = Rc::new(RefCell::new(0));

    let label = Label::builder()
        .label(&number.borrow().to_string())
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let increment = Button::builder()
        .label("Increment")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number_clone = Rc::clone(&number);
    let label_clone = label.clone();

    increment.connect_clicked(move |_| {
        *number_clone.borrow_mut() += 1;
        label_clone.set_label(&number_clone.borrow().to_string());
    });

    let decrement = Button::builder()
        .label("Decrement")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let number_clone = Rc::clone(&number);
    let label_clone = label.clone();

    decrement.connect_clicked(move |_| {
        *number_clone.borrow_mut() -= 1;
        label_clone.set_label(&number_clone.borrow().to_string());
    });

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    label.show();
    increment.show();
    decrement.show();
    vbox.add(&label);
    vbox.add(&increment);
    vbox.add(&decrement);
    vbox.show();
    // Create a window with a default size
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(300) // Set a default width
        .default_height(200) // Set a default height
        .child(&vbox)
        .build();

    window.present();
}