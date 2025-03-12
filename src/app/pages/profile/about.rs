use leptos::*;
use crate::app::pages::profile::section_heading::SectionHeading;
use crate::app::lib::hooks::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};

#[component]
pub fn About() -> impl IntoView {

    let is_visible = use_section_in_view("about", 0.75, 1000.0);

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::About);
        }
    });

    view! {
        <section class="mb-28 max-w-[45rem] text-center leading-8 sm:mb-40 animate-text-animation-fast scroll-mt-28" id="about">
            <SectionHeading>
                "About Me"
            </SectionHeading>
            <p class="mb-3">
                <span class="font-medium">
                    "I am Chan-woong Kim, a software engineer with a passion for in-depth exploration and continuous learning. With a solid foundation in computer science and software engineering, I am driven by a desire to tackle complex technical challenges, particularly in AI and system optimization."
                </span>
            </p>
            <p class="mb-3">
                <span class="font-medium">                
                    "Throughout my journey, I have participated in four competitions, including the KRX National University Student Securities and Derivatives Competition, where I received an encouragement award, showcasing my technical skills and dedication. Currently, I am participating in the Seoul Data Fellowship, where I continue to deepen my expertise in data and software solutions. My focus lies in AI and software development, where I continuously push the boundaries through challenging projects. Using deep learning frameworks like PyTorch, I work with multiple data modules, experimenting with various augmentation techniques to optimize performance."
                </span>
            </p> 
            <p class="mb-3">
                <span class="font-medium">                
                    "In the realm of web development, I actively engage with cutting-edge technologies like Rust and Leptos, implementing features such as user authentication and blog functionalities in web applications. As an engineer dedicated to growth, I’m always ready to take on new challenges and leverage my unique skills to contribute meaningfully to the field."
                </span>
            </p> 
        </section>
    }
}