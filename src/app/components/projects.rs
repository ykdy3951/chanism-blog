use leptos::*;
use serde::Deserialize;
use serde_json;
use super::section_heading::SectionHeading;

use super::project::Project;

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

    let projects_data = r#"
    [
        {
            "title": "CLLM",
            "description": "Empower your CLI experience with a command search tool driven by LLM magic!",
            "tags": ["Rust", "CLI", "LLM"],
            "image": "./cllm.png"
        },
        {
            "title": "Teacher's Pick",
            "description": "Korean high school Korean language subject problem solving service using forgetting curve and ai.",
            "tags": ["Mobile", "AI", "Korean", "Flutter", "NLP"],
            "image": "./teachers_pick.png"
        },
        {
            "title": "Style Shift",
            "description": "Image Style Transfer using Neural Networks",
            "tags": ["Computer Vision", "Neural Networks", "AI"],
            "image": "https://via.placeholder.com/150"
        }
    ]
    "#;

    let projects: Vec<ProjectData> = serde_json::from_str(projects_data).unwrap();

    view! {
        <section id="projects" class="scroll-mt-28">
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