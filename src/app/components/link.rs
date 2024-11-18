use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn Link<F>(
    children: Children,
    href: String,
    class: F,
) -> impl IntoView
where
    F: Fn() -> String + 'static,
{
    let navigate = use_navigate();
    let href_clone = href.clone();
    let on_click = move |_| {
        navigate(
            &href_clone,
            NavigateOptions::default()
        )};

    view! {
        <a href={href} on:click=on_click class=move || {
            class()
        }>
        {
            children()
        }
        </a>
    }
}