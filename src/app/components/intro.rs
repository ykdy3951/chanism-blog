use leptos::*;
use web_sys::console;
use leptos_icons::*;
use icondata as i;
use crate::app::hooks::use_section_in_view::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};

#[component]
pub fn Intro() -> impl IntoView {

    let is_visible = use_section_in_view("home", 0.75, 1000.0);

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Home);
        }
    });

    view! {
        <section class="mb-28 max-w-[50rem] text-center sm:mb-0 scroll-mt-[100rem]" id="home">
            <div class="flex items-center justify-center">
                <div class="relative">
                    <div class="animate-image-animation">
                        <img src="https://avatars.githubusercontent.com/u/48588531?v=4" alt="Chanwoong Kim" width="192" height="192" quality="95" priority=true 
                            class="rounded-full w-24 h-24 object-cover border-[0.35rem] border-white shadow-xl"
                        />
                    </div>
                    <span class="absolute text-3xl bottom-0 right-0 animate-span-animation">"👋"</span>
                </div>
            </div>
            
            <h1 class="mb-10 mt-4 px-4 text-2xl font-medium !leading-[1.5] sm:text-4xl animate-text-animation">
                <span class="font-bold">"Hello, I'm Chanwoong Kim"</span>
                <span>" I'm a "</span>
                <span class="font-bold">"Software Engineer"</span>
                <span>" with a passion for "</span>
                <span class="font-bold">"AI Engineering"</span>
                <span>" and "</span>
                <span class="font-bold">"Web Development."</span>
            </h1>

            <div class="flex flex-col sm:flex-row items-center justify-center gap-2 px-4 text-lg font-medium animate-text-animation">
                <a href="#contact" class="group bg-gray-900 text-white px-7 py-3 flex items-center gap-2 rounded-full outline-none focus:scale-110 hover:scale-110 hover:bg-gray-950 active:scale-105 transition"
                    on:click=|_| {
                        console::log_1(&"Contact button clicked".into());
                    }
                >
                    <span>"Contact Me "</span>
                    <Icon icon=i::BsArrowRight class="opacity-70 group-hover:translate-x-1 transition" />
                </a>
                <a class="bg-white px-7 py-3 flex items-center gap-2 rounded-full outline-none focus:scale-110 hover:scale-110 active:scale-105 transition cursor-pointer border-black/10"
                    href="./chanism_cv.pdf" download=true
                >
                    <span>"Download CV"</span>
                    <Icon icon=i::BsDownload class="opacity-60 group-hover:translate-y-1 transition" />
                </a>
                <a class="flex items-center gap-2 bg-white p-4 rounded-full focus:scale-[1.15] hover:scale-[1.15] active:scale-105 transition cursor-pointer border-black/10"
                    href="https://www.linkedin.com/in/chanwoong-kim-1632b6283/" target="_blank"
                >
                    <Icon icon=i::BsLinkedin class="text-[#0077b5] text-2xl" />
                </a>
                <a class="flex items-center gap-2 bg-white p-4 rounded-full focus:scale-[1.15] hover:scale-[1.15] active:scale-105 transition cursor-pointer border-black/10"
                    href="https://www.github.com/ykdy3951" target="_blank"
                >
                    <Icon icon=i::FaSquareGithubBrands class="text-[#333] text-3xl" />
                </a>
            </div>

        </section>
    }
}