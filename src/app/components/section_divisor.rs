use leptos::*;

#[component]
pub fn SectionDivisor() -> impl IntoView {
    view! {
        <div class="bg-gray-200 my-24 h-[7rem] w-1 rounded-full hidden sm:block animate-text-animation dark:bg-opacity-20"></div>
    }
}