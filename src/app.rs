use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod context;
pub mod api;
pub mod hooks;

use components::header::Header;
use components::intro::Intro;
use components::section_divisor::SectionDivisor;
use components::about::About;
use components::projects::Projects;
use components::test::ScrollingComponent;
use components::skills::Skills;

use context::active_section_context::ActiveSectionContextProvider;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/chanism-blog.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="about" view=AboutPage/>
                    <Route path="test" view=ScrollingComponent/>
                </Routes>
            </main>
        </Router>
    }
}


/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=on_click class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "Click number " {count}
                    </button>
                </div>
            </div>
        </main>
    }
}

/// Renders the about page of your application.
#[component]
fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About Me"/>
        <main class="text-gray-950 relative pt-28 sm:pt-36 h-[5000px]" style="scroll-behavior: smooth;">
            <div class="bg-[#fbe2e3] absolute top-[-6rem] -z-10 right-[11rem] h-[31.25rem] w-[31.25rem] rounded-full blur-[10rem] sm:w-[68.75rem]" />
            <div class="bg-[#dbd7fb] absolute top-[-1rem] -z-10 left-[-35rem] h-[31.25rem] w-[50rem] rounded-full blur-[10rem] sm:w-[68.75rem] md:left-[-33rem] lg:left-[-28rem] xl:left-[-15rem] 2xl:left-[-5rem]"/>
            <ActiveSectionContextProvider>            
                <Header/>
                <div class="flex flex-col items-center px-4">
                    <Intro/>
                    <SectionDivisor/>
                    <About/>
                    <Projects/>
                    <Skills />
                </div>
            </ActiveSectionContextProvider>
        </main>
    }
}
