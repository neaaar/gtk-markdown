use gtk::{AlertDialog, ApplicationWindow};

pub fn show_error_dialog(parent: &ApplicationWindow, message: &str) {
    let alert = AlertDialog::builder().modal(true).message(message).build();

    alert.show(Some(parent));
}
