mod imp;

use gio::SimpleAction;
use glib::{clone, Object};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{Application, Orientation};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }

    fn add_actions(&self) {
        // Get state
        let imp = self.imp();
        let label = imp.label.get();

        // Add stateful action "count" to `window` taking an integer as parameter
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "count",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak label => move |action, parameter| {
            // Get state
            let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<i32>()
            .expect("The variant needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The variant needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            action.set_state(&state.to_variant());

            // Update label with new state
            label.set_label(&format!("Counter: {}", state));
        }));
        self.add_action(&action_count);

        // Add action "close" to `window` taking no parameter
        let action_close = SimpleAction::new("close", None);

        action_close.connect_activate(clone!(@weak self as window => move |_, _| {
            window.close();
        }));
        self.add_action(&action_close);

        // ANCHOR: settings_create_actions
        // Create action from key "sensitive-button" and add to action group "win"
        let action_sensitive_button = imp.settings.create_action("sensitive-button");
        self.add_action(&action_sensitive_button);

        // Create action from key "orientation" and add to action group "win"
        let action_orientation = imp.settings.create_action("orientation");
        self.add_action(&action_orientation);
        // ANCHOR_END: settings_create_actions
    }

    // ANCHOR: bind_settings
    fn bind_settings(&self) {
        // Get state
        let imp = self.imp();

        // Bind setting "sensitive-button" to "sensitive" property of `button`
        let button = imp.button.get();
        imp.settings
            .bind("sensitive-button", &button, "sensitive")
            .build();

        // Bind setting "orientation" to "orientation" property of `button`
        let gtk_box = imp.gtk_box.get();
        imp.settings
            .bind("orientation", &gtk_box, "orientation")
            .mapping(|variant, _| {
                let orientation = variant
                    .get::<String>()
                    .expect("The variant needs to be of type `String`.");

                let orientation = match orientation.as_str() {
                    "Horizontal" => Orientation::Horizontal,
                    "Vertical" => Orientation::Vertical,
                    _ => unreachable!(),
                };

                Some(orientation.to_value())
            })
            .build();
    }
    // ANCHOR_END: bind_settings
}
