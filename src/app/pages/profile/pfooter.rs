use leptos::*;

#[component]
pub fn ProfileFooter(    
) -> impl IntoView {
    view! {
        <footer class="mb-10 px-4 text-center text-gray-500">
            <small class="mb-2 block text-xs">
                "&copy; 2024 Chanwoong Kim. All rights reserved."
            </small>
            <p class="text-xs">
                <span class="font-semibold">"About this website:"</span>
                " built with Leptos, Tailwind CSS, and Rust." 
                <br/>
                "Designed by "<a href="https://github.com/ByteGrad/portfolio-website">"Ricardo."</a>
            </p>
        </footer>
    }
}