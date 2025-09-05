mod callbacks;
mod markdown;
mod preview;
mod ui;
mod ui_helpers;

use gtk::glib;
use gtk::prelude::*;
use gtk::Application;
use ui::build_ui;

const APP_ID: &str = "com.marco.farioli.gtkmarkdown";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        build_ui(app);
    });

    // Run the application
    app.run()
}
