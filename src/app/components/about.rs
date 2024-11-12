use leptos::*;

use super::section_heading::SectionHeading;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="mb-28 max-w-[45rem] text-center leading-8 sm:mb-40 animate-text-animation-fast scroll-mt-28" id="intro">
            <SectionHeading>
                "About Me"
            </SectionHeading>
            <p class="mb-3">
                <span class="font-medium">
                    "I'm Chan-woong Kim, an software engineer with a passion for in-depth exploration in my field."
                </span>
            </p>
            <p class="mb-3">
                <span class="font-medium">                
                    "I have a strong foundation in computer science and software engineering, and I'm always looking for ways to improve my skills."
                </span>
            </p> 
        </section>
    }
}