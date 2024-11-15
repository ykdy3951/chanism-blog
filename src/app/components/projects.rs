use leptos::*;
use serde::Deserialize;
use serde_json;
use crate::app::lib::data::PROJECTS_DATA;
use crate::app::components::project::Project;
use crate::app::components::section_heading::SectionHeading;
use crate::app::lib::hooks::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};

#[derive(Deserialize)]
pub struct ProjectData {
    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "image")]
    pub image: String,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects: Vec<ProjectData> = serde_json::from_str(PROJECTS_DATA).unwrap();
    let is_visible = use_section_in_view("projects", 0.75, 1000.0);

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Projects);
        }
    });

    view! {
        <section id="projects" class="scroll-mt-28 mb-28">
            <SectionHeading>
                "My Projects"
            </SectionHeading>
            <div>
                {
                    // Fragment
                    projects.into_iter().map(|project| view! {
                        <Project
                            project_data=project
                        />
                    })
                        .collect::<Vec<_>>() // Vec<Fragment>
                }
            </div>

        </section>
    }
}