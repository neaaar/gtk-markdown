use glib::{timeout_add_local, ControlFlow, SourceId};
use gtk::prelude::*;
use gtk::TextBuffer;
use std::cell::RefCell;
use std::rc::Rc;
use webkit6::prelude::*;
use webkit6::WebView;

use crate::markdown::to_html;

pub fn update_preview(buffer: &TextBuffer, previewer: &WebView) {
    let debounce_id: Rc<RefCell<Option<SourceId>>> = Rc::new(RefCell::new(None));
    // Making a previewer_rc since we need to clone it both here and inside the first closure
    let previewer_rc = Rc::new(previewer.clone());

    buffer.connect_changed(move |buf| {
        // If there is an active timer, delete it
        if let Some(id) = debounce_id.borrow_mut().take() {
            let _ = id.remove();
        }

        let text = buf
            .text(&buf.start_iter(), &buf.end_iter(), false)
            .to_string();

        // Start new timer (debounce 500ms)
        let previewer_rc = previewer_rc.clone();
        let debounce_id_clone = debounce_id.clone();
        let new_id = timeout_add_local(std::time::Duration::from_millis(300), move || {
            // The preview will update only if this timer runs out
            let html = to_html(&text);
            previewer_rc.load_html(&html, None);

            // Remove expired id from debounce_id
            *debounce_id_clone.borrow_mut() = None;
            ControlFlow::Break
        });

        //Update timer id
        *debounce_id.borrow_mut() = Some(new_id);
    });
}
