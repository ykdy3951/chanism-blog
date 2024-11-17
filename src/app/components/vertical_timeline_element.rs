use leptos::*;
use icondata as i;
use leptos_icons::*;
use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen::JsCast;
use web_sys::js_sys::Array;

use web_sys::MouseEvent;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

use crate::app::lib::utils::class_names;

fn style_to_string(style: Vec<(String, String)>) -> String {
    if style.is_empty() {
        return "".to_string();
    }
    style.iter().map(|(key, value)| format!("{}: {}", key, value)).collect::<Vec<String>>().join("; ")
}

#[component]
pub fn VerticalTimelineElement (
    children: Children,
    id : String,
    #[prop(default = "".to_string())]
    class: String,
    #[prop(optional)]
    content_arrow_style : Vec<(String, String)>,
    #[prop(optional)]
    content_style : Vec<(String, String)>,
    #[prop(default = "".to_string())]
    date: String,
    #[prop(default = "".to_string())]
    date_class : String,
    #[prop(optional)]
    icon: Option<i::Icon>,
    #[prop(default = "".to_string())]
    icon_class: String,
    #[prop(optional)]
    icon_on_click: Option<Box<dyn Fn()>>,
    #[prop(optional)]
    icon_style: Vec<(String, String)>,
    #[prop(optional)]
    on_timeline_element_click: Option<Box<dyn Fn()>>,
    #[prop(default = "".to_string())]
    position: String,
    #[prop(optional)]
    style: Vec<(String, String)>,
    #[prop(default = "".to_string())]
    text_class : String,
    #[prop(default = true)]
    trigger_once: bool,
    #[prop(default = false)]
    visible : bool,
    #[prop(default = "small".to_string())]
    shadow_size: String,
) -> impl IntoView {

    let (is_visible, set_is_visible) = create_signal(false);
    let (is_visible_once, set_is_visible_once) = create_signal(false);
    
    create_effect({
        let element_id = format!("vertical-timeline-element-{}", id);
        move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element = document.get_element_by_id(&element_id).unwrap();

            if is_visible_once.get() && trigger_once {
                return;
            }
            
            let callback = Closure::wrap(Box::new(move |entries: JsValue, _: JsValue| {
                let entries = Array::from(&entries);
                for entry in entries.iter() {
                    let entry = entry.unchecked_into::<IntersectionObserverEntry>();

                    if entry.is_intersecting() {
                        set_is_visible.set(true);
                        if trigger_once {
                            set_is_visible_once.set(true);
                        }
                    } else {
                        set_is_visible.set(false);
                    }
                }
            }) as Box<dyn FnMut(JsValue, JsValue)>);

            let intersection_observer_props = IntersectionObserverInit::new();
            intersection_observer_props.set_root_margin("0px 0px -40px 0px");

            let observer = IntersectionObserver::new_with_options(
                callback.as_ref().unchecked_ref(),
                &intersection_observer_props,
            ).expect("Observer 생성 실패");

            observer.observe(&element);

            on_cleanup(move || {
                observer.disconnect();
                drop(callback);
            });
        }
    });

    view! {
        <div
            id={format!("vertical-timeline-element-{}", id)}
            class=move || {
                class_names(&[class.as_str(), "vertical-timeline-element"], &[
                    ("vertical-timeline-element--left", position == "left"),
                    ("vertical-timeline-element--right", position == "right"),
                    // ("vertical-timeline-element--no-children", child_count.get() == 0 as usize)
                    ])
                }
            style={style_to_string(style)}
        >
            <span
                style={style_to_string(icon_style)}
                class=move || {
                    class_names(&["vertical-timeline-element-icon", icon_class.as_str(), 
                        format!("shadow-size-{}", shadow_size).as_str()
                    ], &[
                        ("bounce-in", is_visible.get() || visible),
                        ("is_hidden", !(is_visible.get() || visible))
                    ])
                }
                on:click=move |_| {
                    if let Some(icon_on_click) = icon_on_click.as_ref() {
                        icon_on_click();
                    }
                }
            >
                {
                    icon.map(|icon| view! { <Icon icon=icon /> })
                }
            </span>
            <div
                class=move ||{
                    class_names(&["vertical-timeline-element-content", text_class.as_str()], &[
                        ("bounce-in", is_visible.get() || visible),
                        ("is_hidden", !(is_visible.get() || visible))
                    ])
                }
                style={style_to_string(content_style)}
                on:click=move |_: MouseEvent| {
                    if let Some(on_timeline_element_click) = on_timeline_element_click.as_ref() {
                        on_timeline_element_click();
                    }
                }
            >
                <div
                    style={style_to_string(content_arrow_style)}
                    class="vertical-timeline-element-content-arrow"  
                />
                {
                    children()
                }
                <span
                    class=format!("vertical-timeline-element-date {}", date_class)
                >
                    {date}
                </span>
            </div>
        </div>
    }
}

