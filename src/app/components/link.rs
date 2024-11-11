use leptos::*;
use leptos_router::{use_navigate, NavigateOptions};

#[component]
pub fn Link(
    #[prop(optional)] 
    children: Option<impl IntoView>,
    href: String,
    class: Option<String>,
) -> impl IntoView {
    let navigate = use_navigate();
    let href_clone = href.clone();
    let on_click = move |_| {
        navigate(
            &href_clone,
            NavigateOptions::default()
        )};

    view! {
        <a href={href} on:click=on_click class={class.unwrap_or_default()}>
            {children}
        </a>
    }
}