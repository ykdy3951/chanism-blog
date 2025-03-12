use leptos::*;
use std::collections::HashMap;

use crate::app::context::active_section_context::{SectionState, section_name_to_string, SectionName};
use crate::app::components::link::Link;

macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

#[component]
pub fn ProfileHeader() -> impl IntoView {

    let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");

    let (active_section, set_active_section) = create_slice(
        state,
        |state: &SectionState| section_name_to_string(state.active_section.clone()),
        |state: &mut SectionState, active_section: String| {
            state.active_section = match active_section.as_str() {
                "Home" => SectionName::Intro,
                "About" => SectionName::About,
                "Projects" => SectionName::Projects,
                "Skills" => SectionName::Skills,
                "Experience" => SectionName::Experience,
                "Contact" => SectionName::Contact,
                _ => SectionName::Intro,
            };
        }
    );

    let data = vec![
        hashmap! {
            "name" => "Blog",
            "hash" => "/",
        },
        hashmap! {
            "name" => "Intro",
            "hash" => "/profile#intro",
        },
        hashmap! {
            "name" => "About",
            "hash" => "/profile#about",
        },
        hashmap! {
            "name" => "Projects",
            "hash" => "#projects",
        },
        hashmap! {
            "name" => "Skills",
            "hash" => "/profile#skills",
        },
        hashmap! {
            "name" => "Experience",
            "hash" => "/profile#experience",
        },
        hashmap! {
            "name" => "Contact",
            "hash" => "/profile#contact",
        }
    ];

    let on_click = move |item_name: String| {
        // _set_active_section.update(|active_section| *active_section = item_name.to_string());
        set_active_section(item_name.to_string());
    };
    
    view! {
        <header class="z-[999] relative">
            <div class="fixed top-0 left-1/2 h-[4.5rem] w-full rounded-none border border-white border-opacity-40 bg-white bg-opacity-80 shadow-lg shadow-black/[0.03] backdrop-blur-[0.5rem] sm:top-6 sm:h-[3.25rem] sm:w-[40rem] sm:rounded-full animate-slide-down dark:bg-gray-950 dark:border-black/40 dark:bg-opacity-75"></div>

            <nav class="flex fixed top-[0.15rem] left-1/2 h-12 -translate-x-1/2 py-2 sm:top-[1.7rem] sm:h-[initial] sm:py-0">
                <ul class="flex w-[22rem] flex-wrap items-center justify-center gap-y-1 text-[0.9rem] font-medium text-gray-500 sm:w-[initial] sm:flex-nowrap sm:gap-5">
                    {
                        data.into_iter()
                            .map(|item| {
                                let item_name = item["name"].to_string();
                                let item_hash = item["hash"].to_string();
                                let item_name_clone = item_name.clone();
                                let item_name_for_signal = item_name.clone();
                                let is_active = create_memo({
                                    let active_section = active_section.clone();
                                    move |_| item_name_for_signal == *active_section.get()
                                });
                                
                                view! {
                                    <li key=item_hash.clone() class="h-3/4 flex items-center justify-center animate-slide-down-no-translate-x">
                                        <Link 
                                            href=format!("{}", item_hash)
                                            // class="flex w-full items-center justify-center px-3 py-3 hover:text-gray-950 transition dark:text-gray-500 dark:hover:text-gray-300".to_string()
                                            // condition_class=("text-gray-950 dark:!text-gray-300".to_string(), Signal::derive(is_active))
                                            class=move || {
                                                if is_active() {
                                                    "flex w-full items-center justify-center px-3 py-3 text-gray-950 dark:text-gray-300 hover:text-gray-950 dark:hover:text-gray-300 transition".to_string()
                                                } else {
                                                    "flex w-full items-center justify-center px-3 py-3 text-gray-500 dark:text-gray-500 hover:text-gray-950 dark:hover:text-gray-300 transition".to_string()
                                                }
                                            }
                                            on:click= move |_| { 
                                                on_click(item_name_clone.clone());
                                            }
                                            
                                        >
                                            {item_name.clone()}
                                            {
                                                move || (
                                                    if is_active() {
                                                        view! {<span class="bg-gray-100 rounded-full absolute inset-0 -z-10 transition animate-slide-horizontal dark:bg-gray-800"></span>}
                                                    } else {
                                                        view! {<span class="bg-transparent rounded-full absolute inset-0 -z-10"></span>}
                                                    }
                                                )
                                            }
                                            
                                        </Link>
                                    </li>}
                            })
                            .collect::<Vec<_>>()
                    }
                </ul>
            </nav>
        </header>
    }
}