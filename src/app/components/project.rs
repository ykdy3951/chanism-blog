use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use crate::app::components::projects::ProjectData;

#[component]
pub fn Project(
    project_data: ProjectData
) -> impl IntoView {
    // how to use ref
    let (style, set_style) = create_signal("".to_string());
    let element_id = project_data.title.clone();

    let on_scroll = move |_: web_sys::Event| {
        if let Some(window) = window() {
            let document = window.document().unwrap();
            let target_element : web_sys::Element = document.get_element_by_id(&element_id).unwrap().dyn_into().unwrap();

            let element_top = target_element.get_bounding_client_rect().top();
            let viewport_height = window.inner_height().unwrap().as_f64().unwrap();
            let element_height = target_element.scroll_height() as f64;
            
            let progress = ((viewport_height - element_top) / (element_height * 1.33)).clamp(0.0, 1.0);
            let new_scale = 0.8 + 0.2 * progress;
            let new_opacity = 0.6 + 0.4 * progress;

            set_style.update(|style| *style = format!("transform: scale({}); opacity: {}", new_scale, new_opacity));
        }
    };

    create_effect(move |_| {
        if let Some(window) = window() {
            let closure = Closure::wrap(Box::new(on_scroll.clone()) as Box<dyn FnMut(_)>);
            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    });

    view! {
        <div style={style.clone()} class="group mb-3 sm:mb-8 last:mb-0" id={project_data.title.clone()}>
            <section class="bg-gray-100 max-w-[42rem] border border-black/5 overflow-hidden sm:pr-8 relative sm:h-[20rem] sm:w-[42rem]
                even:pl-8 hover:bg-gray-200 transition group-even:pl-8 rounded-lg">
                <div class="pt-4 pb-8 px-5 sm:pl-10 sm:pr-2 sm:pt-10 sm:max-w-[50%] flex flex-col h-full group-even:ml-[18rem]">
                    <h3 class="text-2xl font-semibold">{project_data.title.clone()}</h3>
                    <p class="mt-2 leading-relaxed text-gray-700">{project_data.description}</p>
                    <ul class="flex flex-wrap mt-4 gap-2 sm:mt-auto">
                    {
                        project_data.tags.into_iter().enumerate().map(|(idx, tag)| view! {
                            <li class="bg-black/[0.7] px-3 py-1 text-[0.7rem] uppercase tracking-wider text-white rounded-full" key={idx}>{tag}</li>
                        })
                            .collect::<Vec<_>>()
                    }
                    </ul>
                </div>
                <img src={project_data.image} alt={project_data.title} quality={95} 
                    class="absolute top-8 -right-40 w-[28.25rem] rounded-t-lg shadow-2xl 
                    transition group-hover:scale-[1.04] group-hover:-translate-x-3 group-hover:translate-y-3 group-hover:-rotate-2 
                    group-even:group-hover:translate-x-3 group-even:group-hover:-translate-y-3 group-even:group-hover:rotate-2
                    group-even:right-[initial] group-even:-left-40"
                />
            </section>
        </div>
    }
}

