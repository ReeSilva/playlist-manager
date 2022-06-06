use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, InputPurpose};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("com.reesilva.playlist-manager")
        .build();
    
    //Connect to "activate" signal of the `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let username = Entry::builder()
        .name("Username")
        .has_frame(true)
        .input_purpose(InputPurpose::Name)
        .max_length(32)
        .placeholder_text("Entry your username here")
        .build();
    
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // Connect to "clicked" signal of `button`
    button.connect_clicked(move |button| {
        let username_input = username.to_string();
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label(&username_input);
    });
    
    // Create a window and set the title
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Playlist Manager")
        .child(&button)
        .build();

    // Present Window
    window.present();
}