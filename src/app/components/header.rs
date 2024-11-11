use leptos::*;
use std::collections::HashMap;

use super::link::Link;

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
pub fn Header() -> impl IntoView {

    let data = vec![
        hashmap! {
            "name" => "Home",
            "hash" => "#home",
        },
        hashmap! {
            "name" => "About",
            "hash" => "#intro",
        },
        hashmap! {
            "name" => "Projects",
            "hash" => "#projects",
        },
        hashmap! {
            "name" => "Skills",
            "hash" => "#skills",
        },
        hashmap! {
            "name" => "Experience",
            "hash" => "#experience",
        },
        hashmap! {
            "name" => "Contact",
            "hash" => "#contact",
        }
    ];


    view! {
        <header class="z-[999] relative">
            <div class="fixed top-0 left-1/2 h-[4.5rem] w-full rounded-none border border-white border-opacity-40 bg-white bg-opacity-80 shadow-lg shadow-black/[0.03] backdrop-blur-[0.5rem] sm:top-6 sm:h-[3.25rem] sm:w-[36rem] sm:rounded-full animate-slide-down"></div>

            <nav class="flex fixed top-[0.15rem] left-1/2 h-12 -translate-x-1/2 py-2 sm:top-[1.7rem] sm:h-[initial] sm:py-0">
                <ul class="flex w-[22rem] flex-wrap items-center justify-center gap-y-1 text-[0.9rem] font-medium text-gray-500 sm:w-[initial] sm:flex-nowrap sm:gap-5">
                    {
                        data.into_iter()
                            .map(|item| view! {
                            <li key=item["hash"] class="h-3/4 flex items-center justify-center animate-slide-down-no-translate-x">
                                <Link 
                                    href=format!("/about{}", item["hash"].to_string())
                                    children=item["name"].to_string() 
                                    class=Some("flex w-full items-center justify-center px-3 py-3 hover:text-gray-950 transition".to_string())
                                />
                            </li>})
                            .collect::<Vec<_>>()
                    }
                </ul>
            </nav>
        </header>
    }
}