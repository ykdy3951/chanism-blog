use leptos::*;
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::js_sys::{self, Array};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use std::cell::Cell;
use std::rc::Rc;

pub fn use_section_in_view(
    element_id: &str,
    threshold: f64,
    visibility_delay_ms: f64,
) -> Signal<bool> {
    let (is_visible, set_is_visible) = create_signal(false);
    let time_of_last_interaction = Rc::new(Cell::new(0));

    create_effect({
        let element_id = element_id.to_string();

        move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element = document.get_element_by_id(&element_id).unwrap();
            let last_interaction: Rc<Cell<u32>> = time_of_last_interaction.clone();

            let callback = Closure::wrap(Box::new(move |entries: JsValue, _: JsValue| {
                let entries = Array::from(&entries);
                for entry in entries.iter() {
                    let entry = entry.unchecked_into::<IntersectionObserverEntry>();
                    let current_time = js_sys::Date::now();

                    if entry.is_intersecting() && (current_time - last_interaction.get() as f64 > visibility_delay_ms) {
                        set_is_visible.set(true);
                        last_interaction.set(current_time as u32);
                    } else {
                        set_is_visible.set(false);
                    }
                }
            }) as Box<dyn FnMut(JsValue, JsValue)>);

            let options = IntersectionObserverInit::new();
            options.set_threshold(&JsValue::from_f64(threshold));

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &options,
            ).expect("Failed to create IntersectionObserver");

            observer.observe(&element);

            on_cleanup(move || {
                observer.disconnect();
                drop(callback);
            });
        }
    });

    is_visible.into()
}
