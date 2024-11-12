use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn Link(
    children: Children,
    href: String,
    class: Option<String>,
    style: Signal<bool>,
) -> impl IntoView {
    let navigate = use_navigate();
    let href_clone = href.clone();
    let on_click = move |_| {
        navigate(
            &href_clone,
            NavigateOptions::default()
        )};

    view! {
        <a href={href} on:click=on_click class={class.unwrap_or_default()}
            style=move || {
                if style.get() {
                    "--tw-text-opacity: 1; color: rgb(3 7 18 / var(--tw-text-opacity))".to_string()
                } else {
                    "".to_string()
                }
            }
        >
            {
                children()
            }
        </a>
    }
}