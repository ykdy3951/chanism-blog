use std::rc::Rc;
use std::cell::Cell;
use leptos::logging::log;
use wasm_bindgen::JsCast;

use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsValue;
use web_sys::{js_sys, Document, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use crate::app::components::section_heading::SectionHeading;
use crate::app::lib::hooks::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};
use crate::app::lib::data::SKILLS_DATA;

fn add_dynamic_styles(document: &Document, num_skills: usize, base_delay: f64) {
    let style_element = document.create_element("style").unwrap();
    let mut styles = String::new();

    styles.push_str(
        "@keyframes spring-up {
            0% {
                transform: translateY(110px) scale3d(0.96,0.96,1);
                opacity: 0;
            }
            60% {
                transform: translateY(-10px);
                opacity: 1;
            }
            80% {
                transform: translateY(10px);
            }
            100% {
                transform: translateY(0) scale3d(1,1,1);
            }
        }

        .skill-item {
            opacity: 0;
            transform: translateY(100px);
            transition: opacity 0.5s ease, transform 0.5s ease;
        }
        \n",
    );

    for i in 0..num_skills {
        let delay = base_delay * (i as f64);
        styles.push_str(&format!(
            ".fade-in-delay-{} {{
                animation: spring-up 2s cubic-bezier(0.25, 1.5, 0.5, 1) forwards;
                animation-delay: {}s;
            }}\n",
            i, delay
        ));
    }

    style_element.set_inner_html(&styles);
    document.head().unwrap().append_child(&style_element).unwrap();
}

#[component]
pub fn Skills() -> impl IntoView {

    let is_visible = use_section_in_view("skills", 0.75, 1000.0);
    let is_once_triggered = Rc::new(Cell::new(false));
    let (style_name, set_style_name) = create_signal("skill-item".to_string());

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Skills);
        }
        
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let section = document.get_element_by_id("skills").unwrap();
        let is_once_triggered: Rc<Cell<bool>> = is_once_triggered.clone();
        
        let num_skills = SKILLS_DATA.len();
        let base_delay = 0.08;
        
        add_dynamic_styles(&document, num_skills, base_delay);

        let observer_callback = 
            Closure::wrap(Box::new(move |entries: JsValue, _: JsValue| {
                let entries = js_sys::Array::from(&entries);
                for entry in entries.iter() {
                    let entry = entry.unchecked_into::<IntersectionObserverEntry>();

                    if entry.is_intersecting() && !is_once_triggered.get() {
                        log!("skills section is intersecting");
                        is_once_triggered.set(true);
                        set_style_name.update(|style_name| *style_name = "fade-in-delay".to_string());
                    }
                }
            }) as Box<dyn FnMut(JsValue, JsValue)>);

        let options = IntersectionObserverInit::new();
        options.set_threshold(&JsValue::from_f64(0.5));
        
        let observer = IntersectionObserver::new_with_options(observer_callback.as_ref().unchecked_ref(), &options)
            .expect("failed to create IntersectionObserver");

        observer.observe(&section);

        on_cleanup(move || {
            observer.disconnect();
            drop(observer_callback);
        });
    });

    view! {
        <section id="skills" class="mb-28 max-w-[53rem] scroll-mt-28 text-center sm:mb-40">
            <SectionHeading>Skills</SectionHeading>
            <ul class="flex flex-wrap justify-center gap-2 text-lg text-gray-800">
                {
                    
                    SKILLS_DATA
                    .into_iter().enumerate().map(|(idx, skill)| 
                    {
                        view! {
                            <li class=move || format!("bg-white borderBlack rounded-xl px-5 py-3 shadow-md {:}", 
                                if style_name.get() == "skill-item" { "skill-item".to_string() } else { format!("fade-in-delay-{}", idx) }
                            ) key={idx}>
                                {*skill}
                            </li>
                        }
                    }).collect::<Vec<_>>()
                }
            </ul>
        </section>
    }
}