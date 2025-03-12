use leptos::*;
use crate::app::pages::profile::pheader::ProfileHeader;
use crate::app::pages::profile::intro::Intro;
use crate::app::pages::profile::section_divisor::SectionDivisor;
use crate::app::pages::profile::about::About;
use crate::app::pages::profile::projects::Projects;
use crate::app::pages::profile::skills::Skills;
use crate::app::pages::profile::experience::Experience;
use crate::app::pages::profile::contact::Contact;
use crate::app::pages::profile::pfooter::ProfileFooter;

use crate::app::context::active_section_context::ActiveSectionContextProvider;

#[component]
pub fn ProfilePage() -> impl IntoView {
    view! {
        <main class="text-gray-950 relative pt-28 sm:pt-36 dark:text-gray-50 dark:text-opacity-90" style="scroll-behavior: smooth;">
            <div class="bg-[#fbe2e3] absolute top-[-6rem] -z-10 right-[11rem] h-[31.25rem] w-[31.25rem] rounded-full blur-[10rem] sm:w-[68.75rem] dark:bg-[#946263]" />
            <div class="bg-[#dbd7fb] absolute top-[-1rem] -z-10 left-[-35rem] h-[31.25rem] w-[50rem] rounded-full blur-[10rem] sm:w-[68.75rem] md:left-[-33rem] lg:left-[-28rem] xl:left-[-15rem] 2xl:left-[-5rem] dark:bg-[#676394]"/>
            <ActiveSectionContextProvider>            
                <ProfileHeader/>
                <div class="flex flex-col items-center px-4">
                    <Intro/>
                    <SectionDivisor/>
                    <About/>
                    <Projects/>
                    <Skills />
                    <Experience />
                    <Contact />
                    <ProfileFooter/>
                </div>
            </ActiveSectionContextProvider>
        </main>
    }
}