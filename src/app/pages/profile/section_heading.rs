use leptos::*;

#[component]
pub fn SectionHeading(
    children: Children,
) -> impl IntoView {
    view! {
        <h2 class="text-3xl font-medium capitalize mb-8 text-center">
            {
                children()
            }
        </h2>
    }
}