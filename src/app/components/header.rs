use leptos::*;
use leptos_router::*;

#[component]
pub fn Header() -> impl IntoView {

    view! {
        <header class="fixed top-0 left-0 right-0 z-50 bg-white dark:bg-gray-950 border-b border-gray-200 dark:border-gray-800">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex items-center justify-between h-16">
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white cursor-pointer hover:text-gray-700 dark:hover:text-gray-300 hover:cursor-pointer"
                        on:click=move |_| {
                            let navigate = use_navigate();
                            navigate("/", Default::default());
                        }
                        >
                        "Chanism Blog"
                    </h1>

                    <div class="flex items-center gap-4">
                        <div class="px-4 py-2">
                            <span class="hover:text-gray-600 cursor-pointer"
                                on:click=move |_| {
                                    let navigate = use_navigate();
                                    navigate("/posts", Default::default());
                                }
                                >
                                "Posts"
                            </span>
                        </div>
                        <button class="px-4 py-2">
                            <span class="hover:text-gray-600 cursor-pointer"
                                on:click=move |_| {
                                    let navigate = use_navigate();
                                    navigate("/profile", Default::default());
                                }
                                >
                                "About"
                            </span>
                        </button>
                        <button class="px-4 py-2 rounded-lg border hover:bg-gray-50 transition-colors">
                            "Login"
                        </button>
                    </div>
                </div>
            </div>
        </header>
    }
}