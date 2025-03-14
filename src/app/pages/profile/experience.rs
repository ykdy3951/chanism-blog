use leptos::*;
use crate::app::pages::profile::section_heading::SectionHeading;
use crate::app::lib::hooks::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};
use crate::app::components::vertical_timeline::VerticalTimeline;
use crate::app::components::vertical_timeline_element::VerticalTimelineElement;
use crate::app::lib::data::EXPERIENCES_DATA;

use crate::app::context::theme_context::{ThemeName, ThemeState};

#[component]
pub fn Experience(
) -> impl IntoView {

    let is_visible = use_section_in_view("experience", 0.75, 1000.0);
    let theme = use_context::<RwSignal<ThemeState>>().expect("ThemeContextProvider not found");

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Experience);
        }
    });

    view! {
        <section id="experience" class="scroll-mt-28 mb-28 sm:mb-40 w-full">
            <SectionHeading>My Experience</SectionHeading>
            <VerticalTimeline
                line_color="".to_string()
            >
                {
                    EXPERIENCES_DATA
                    .into_iter().enumerate().map(|(idx, experience)| 
                    {
                        view! {
                            <VerticalTimelineElement
                                id={idx.to_string()}
                                date={experience.date.to_string()}
                                icon={experience.icon}
                                content_style=Box::new(move || {
                                    vec![
                                        ("background".to_string(), match theme.get().theme {
                                            ThemeName::Light => "#f3f4f6".to_string(),
                                            ThemeName::Dark => "rgba(255, 255, 255, 0.05)".to_string(),
                                        }),
                                        ("box-shadow".to_string(), "0.1rem 0.1rem #ddd".to_string()),
                                        ("border".to_string(), "1px solid rgba(0, 0, 0, 0.05)".to_string()),
                                        ("text-align".to_string(), "left".to_string()),
                                        ("padding".to_string(), "1.3rem 2rem".to_string()),
                                    ]
                                })
                                content_arrow_style=Box::new(move || {
                                    vec![
                                        ("border-right".to_string(), match theme.get().theme {
                                            ThemeName::Light => "0.4rem solid #9ca3af".to_string(),
                                            ThemeName::Dark =>  "0.4rem solid rgba(255, 255, 255, 0.05)".to_string()
                                        })
                                    ]
                                })
                                icon_style=Box::new(move || {
                                    vec![
                                        ("background".to_string(),match theme.get().theme {
                                            ThemeName::Light => "white".to_string(),
                                            ThemeName::Dark => "rgba(255, 255, 255, 0.15)".to_string(),
                                        }),
                                        ("font-size".to_string(), "1.5rem".to_string())
                                    ]
                                })
                            >
                                <h3 class="font-semibold capitalize">{experience.title}</h3>
                                <p class="font-normal !mt-0">{experience.location}</p>
                                <p class="!mt-1 font-normal text-gray-700 dark:text-white/75">
                                    {experience.description}
                                </p>
                            </VerticalTimelineElement>
                        }
                    }).collect::<Vec<_>>()
                }
            </VerticalTimeline>
        </section>
    }

}