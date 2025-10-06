use glib::{timeout_add_local, ControlFlow, SourceId};
use gtk::prelude::*;
use gtk::{TextBuffer, TextIter};
use std::cell::RefCell;
use std::rc::Rc;
use webkit6::prelude::*;
use webkit6::{gio, WebView};

use crate::markdown::to_html;

pub fn update_preview(buffer: &TextBuffer, previewer: &WebView) {
    let debounce_id: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));
    let previewer_rc = Rc::new(previewer.clone());
    let buffer_rc = Rc::new(buffer.clone());

    buffer.connect_changed(move |buf| {
        // If there is an active timer, remove it (reset debounce)
        if let Some(id) = debounce_id.borrow_mut().take() {
            let _ = id.remove();
        }

        let text = buf
            .text(&buf.start_iter(), &buf.end_iter(), false)
            .to_string();

        let previewer_rc = previewer_rc.clone();
        let buffer_rc = buffer_rc.clone();
        let debounce_id_clone = debounce_id.clone();

        // Start a new debounce timer (300ms)
        let new_id = timeout_add_local(std::time::Duration::from_millis(300), move || {
            // Get current cursor position in the TextBuffer
            let cursor_iter: TextIter = buffer_rc.iter_at_mark(&buffer_rc.get_insert());
            let cursor_line = cursor_iter.line();

            // Insert an invisible marker into the text at the cursor line and convert it to HTML
            let text_with_marker = insert_marker(&text, cursor_line);
            let html = to_html(&text_with_marker);

            // Load the new HTML content
            previewer_rc.load_html(&html, None);

            // Once loaded, scroll the WebView to the marker position
            let handler_id = Rc::new(RefCell::new(None));
            let handler_id_clone = handler_id.clone();

            let id = previewer_rc.connect_load_changed(move |wv, event| {
                use webkit6::LoadEvent;
                if event == LoadEvent::Finished {
                    let js = r#"
                        const el = document.getElementById('cursor-pos');
                        if (el) el.scrollIntoView({behavior: 'auto', block: 'center'});
                    "#;
                    wv.evaluate_javascript(&js, None, None, None::<&gio::Cancellable>, |_| {});
                    // Disconnect handler after first run
                    if let Some(handler) = handler_id_clone.borrow_mut().take() {
                        wv.disconnect(handler);
                    }
                }
            });

            *handler_id.borrow_mut() = Some(id);

            // Clear debounce state
            *debounce_id_clone.borrow_mut() = None;
            ControlFlow::Break
        });

        *debounce_id.borrow_mut() = Some(new_id);
    });
}

/// Inserts an invisible marker at the given line index.
/// This marker becomes an <a id="cursor-pos"></a> in the HTML.
fn insert_marker(text: &str, line_index: i32) -> String {
    let mut lines: Vec<&str> = text.lines().collect();
    if line_index as usize >= lines.len() {
        // Append marker at the end if cursor is beyond last line
        lines.push("<a id=\"cursor-pos\"></a>");
    } else {
        // Insert marker at the cursor line
        let mut line = lines[line_index as usize].to_string();
        line.push_str("\n<a id=\"cursor-pos\"></a>");
        lines[line_index as usize] = Box::leak(line.into_boxed_str());
    }
    lines.join("\n")
}
