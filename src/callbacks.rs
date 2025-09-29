use gtk::gio::{Cancellable, ListStore, SimpleAction};
use gtk::prelude::*;
use gtk::{ApplicationWindow, FileDialog, FileFilter, TextBuffer};
use std::fs;

use crate::ui_helpers::show_error_dialog;

pub fn setup_open_action(window: &ApplicationWindow, editor_buffer: &TextBuffer) {
    if let Some(action) = window
        .application()
        .and_then(|app| app.lookup_action("open"))
    {
        if let Some(simple_action) = action.downcast_ref::<SimpleAction>() {
            let window_clone = window.clone();
            let buffer_clone = editor_buffer.clone();

            simple_action.connect_activate(move |_, _| {
                let dialog = FileDialog::builder().title("Open file").build();

                let filter_md_txt = FileFilter::new();
                filter_md_txt.set_name(Some(".md and .txt files"));
                filter_md_txt.add_pattern("*.md");
                filter_md_txt.add_pattern("*.txt");

                let filter_store = ListStore::new::<FileFilter>();
                filter_store.append(&filter_md_txt);
                dialog.set_filters(Some(&filter_store));

                let buffer_clone = buffer_clone.clone();
                let window_for_dialog = window_clone.clone();
                dialog.open(
                    Some(&window_clone),
                    Some(&Cancellable::new()),
                    move |result| {
                        if let Ok(file) = result {
                            if let Some(path) = file.path() {
                                // Check extensions
                                let allowed = ["md", "txt"];
                                if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                                    if !allowed.contains(&ext) {
                                        show_error_dialog(
                                            &window_for_dialog,
                                            &format!("File type not supported: .{}", ext),
                                        );
                                        return;
                                    }
                                } else {
                                    show_error_dialog(
                                        &window_for_dialog,
                                        "File has no extension and cannot be opened.",
                                    );
                                    return;
                                }

                                // If extensions are ok, read file
                                match fs::read_to_string(&path) {
                                    Ok(content) => {
                                        buffer_clone.set_text(&content);
                                    }
                                    Err(err) => {
                                        show_error_dialog(
                                            &window_for_dialog,
                                            &format!("Failed to read file:\n{err}"),
                                        );
                                    }
                                }
                            }
                        }
                    },
                );
            });
        }
    }
}

pub fn setup_save_action(window: &ApplicationWindow, editor_buffer: &TextBuffer) {
    if let Some(action) = window
        .application()
        .and_then(|app| app.lookup_action("save"))
    {
        if let Some(simple_action) = action.downcast_ref::<SimpleAction>() {
            let window_clone = window.clone();
            let buffer_clone = editor_buffer.clone();

            simple_action.connect_activate(move |_, _| {
                let dialog = FileDialog::builder()
                    .title("Save file")
                    .accept_label("Save")
                    .build();

                let buffer_clone = buffer_clone.clone();
                let window_for_dialog = window_clone.clone();
                dialog.save(
                    Some(&window_clone),
                    Some(&Cancellable::new()),
                    move |result| {
                        if let Ok(file) = result {
                            if let Some(mut path) = file.path() {
                                // Add or change extension to .md
                                if path.extension().is_none() || path.extension().unwrap() != "md" {
                                    path.set_extension("md");
                                }

                                let text = buffer_clone
                                    .text(
                                        &buffer_clone.start_iter(),
                                        &buffer_clone.end_iter(),
                                        false,
                                    )
                                    .to_string();

                                if let Err(err) = fs::write(&path, text) {
                                    show_error_dialog(
                                        &window_for_dialog,
                                        &format!("Failed to write file:\n{err}"),
                                    );
                                }
                            }
                        }
                    },
                );
            });
        }
    }
}
