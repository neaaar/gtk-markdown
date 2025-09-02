use gtk::prelude::*;
use gtk::{
    gio, gio::SimpleAction, Application, ApplicationWindow, Box as GtkBox, HeaderBar, MenuButton,
    Orientation, PopoverMenu, ScrolledWindow, TextView,
};
use webkit6::prelude::*;
use webkit6::WebView;

use crate::callbacks::{setup_open_action, setup_save_action};
use crate::markdown::to_html;

pub fn build_ui(app: &Application) {
    let editor = TextView::builder().top_margin(2).left_margin(2).build();
    let previewer = WebView::builder().build();

    // Initialize previewer with empty html
    previewer.load_html("<html><body style='background:white;'></body></html>", None);

    let editor_buffer = editor.buffer();
    let previewer_clone = previewer.clone();

    editor_buffer.connect_changed(move |buf| {
        // 1) Read everything from the buffer
        let text = buf
            .text(&buf.start_iter(), &buf.end_iter(), false)
            .to_string();

        // 2) Convert it to HTML
        let html = to_html(&text);

        // 3) Update previewer (with pure HTML for now)
        previewer_clone.load_html(&html, None);
    });

    let scroll_editor = ScrolledWindow::builder()
        .child(&editor)
        .hexpand(true)
        .vexpand(true)
        .build();
    let scroll_previewer = ScrolledWindow::builder()
        .child(&previewer)
        .hexpand(true)
        .vexpand(true)
        .build();

    let widget_box = GtkBox::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .build();
    widget_box.append(&scroll_editor);
    widget_box.append(&scroll_previewer);

    //HeaderBar witb Open/Save MenuBar
    let open_action = SimpleAction::new("open", None);
    let save_action = SimpleAction::new("save", None);

    app.add_action(&open_action);
    app.add_action(&save_action);

    let menu = gio::Menu::new();
    menu.append(Some("Open"), Some("app.open"));
    menu.append(Some("Save"), Some("app.save"));

    let popover_menu = PopoverMenu::from_model(Some(&menu));
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .popover(&popover_menu)
        .build();

    let header_bar = HeaderBar::builder().show_title_buttons(true).build();
    header_bar.pack_start(&menu_button);

    let main_box = GtkBox::builder().orientation(Orientation::Vertical).build();
    main_box.append(&widget_box);

    // Create a window and set the titlebar
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK Markdown")
        .default_width(1280)
        .default_height(720)
        .child(&main_box)
        .build();
    window.set_titlebar(Some(&header_bar));

    setup_open_action(&window, &editor_buffer);
    setup_save_action(&window, &editor_buffer);

    // Present window
    window.present();
}
