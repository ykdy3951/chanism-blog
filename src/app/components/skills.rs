use leptos::*;
use crate::app::components::section_heading::SectionHeading;
use crate::app::hooks::use_section_in_view::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};

#[component]
pub fn Skills() -> impl IntoView {

    let is_visible = use_section_in_view("skills", 0.75, 1000.0);

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Skills);
        }
    });

    view! {
        <section id="skills" class="mb-28 max-w-[53rem] scroll-mt-28 text-center sm:mb-40">
            <SectionHeading>Skills</SectionHeading>
        </section>
    }
}