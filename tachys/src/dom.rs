use std::cell::RefCell;

use js_sys::JsString;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Window};

thread_local! {
    pub(crate) static WINDOW: RefCell<web_sys::Window> = RefCell::new(
        web_sys::window()
        .unwrap_or(
            JsString::from("window not available").unchecked_into())
        );

    pub(crate) static DOCUMENT: RefCell<web_sys::Document> = {
        let d = WINDOW.with_borrow(|w| w.document())
            .unwrap_or(JsString::from("window has no document").unchecked_into());
        RefCell::new(d)
    };
}

/// Returns the [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window).
///
/// This is cached as a thread-local variable, so calling `window()` multiple times
/// requires only one call out to JavaScript.
pub fn window() -> Window {
    WINDOW.with_borrow(Window::clone)
}

/// Set the [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window).
pub fn set_custom_window_and_document(w: web_sys::Window) {
    DOCUMENT.set(w.document().expect("window has no document"));
    WINDOW.set(w);
}

/// Returns the [`Document`](https://developer.mozilla.org/en-US/docs/Web/API/Document).
///
/// This is cached as a thread-local variable, so calling `document()` multiple times
/// requires only one call out to JavaScript.
///
/// ## Panics
/// Panics if called outside a browser environment.
pub fn document() -> Document {
    DOCUMENT.with_borrow(Document::clone)
}

/// The `<body>` element.
///
/// ## Panics
/// Panics if there is no `<body>` in the current document, or if it is called outside a browser
/// environment.
pub fn body() -> HtmlElement {
    document().body().unwrap()
}

/// Helper function to extract [`Event.target`](https://developer.mozilla.org/en-US/docs/Web/API/Event/target)
/// from any event.
pub fn event_target<T>(event: &web_sys::Event) -> T
where
    T: JsCast,
{
    event.target().unwrap().unchecked_into::<T>()
}

/// Helper function to extract `event.target.value` from an event.
///
/// This is useful in the `on:input` or `on:change` listeners for an `<input>` element.
pub fn event_target_value<T>(event: &T) -> String
where
    T: JsCast,
{
    event
        .unchecked_ref::<web_sys::Event>()
        .target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}

/// Helper function to extract `event.target.checked` from an event.
///
/// This is useful in the `on:change` listeners for an `<input type="checkbox">` element.
pub fn event_target_checked(ev: &web_sys::Event) -> bool {
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .checked()
}
