use gtk::prelude::*;
use gtk::{Align, Box, Orientation, Switch};
use gtk::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

// ANCHOR: activate
fn build_ui(app: &Application) {
    // ANCHOR: switch
    // Create the switch
    let switch = Switch::new();

    // Set and then immediately obtain state
    switch.set_state(true);
    let current_state = switch.state();

    // This prints: "The current state is true"
    println!("The current state is {}", current_state);
    // ANCHOR_END: switch

    // Set up box
    let gtk_box = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&switch);

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // Present the window
    window.present();
}
// ANCHOR_END: activate
