use leptos::*;
use leptos_icons::*;
use icondata as i;

use crate::app::context::theme_context::{ThemeName, ThemeState};


#[component]
pub fn ThemeSwitch() -> impl IntoView {

    let theme = use_context::<RwSignal<ThemeState>>().expect("ThemeContextProvider not found");

    view! {
        <button
            class="fixed bottom-9 right-9 bg-white w-[3.5rem] h-[3.5rem] bg-opacity-80 backdrop-blur-[0.5rem] border border-white border-opacity-40 shadow-2xl rounded-full flex items-center justify-center hover:scale-[1.15] active:scale-105 transition-all dark:bg-gray-950"
            on:click=move |_| {
                match theme.get().theme {
                    ThemeName::Light => theme.set(ThemeState { theme: ThemeName::Dark }),
                    ThemeName::Dark => theme.set(ThemeState { theme: ThemeName::Light }),
                }
            }
        >
        {
            move || {
                match theme.get().theme {
                    ThemeName::Light => view! { <Icon icon=i::BsSunFill class="text-yellow-500 text-xl" /> },
                    ThemeName::Dark => view! { <Icon icon=i::BsMoonStarsFill class="text-yellow-500 text-xl" /> },
                }
            }
        }
        </button>
    }
}