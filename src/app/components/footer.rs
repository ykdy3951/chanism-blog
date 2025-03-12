use leptos::*;

// Blog footer
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="border-t border-gray-200 bg-gray-50 mt-12">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                    <div>
                        <h2 class="text-xl font-bold mb-4">
                            "Chanism Blog"
                        </h2>
                        <p class="text-gray-600 max-w-md">
                            "Your source for the latest insights in technology, design, and engineering. Stay updated with cutting-edge developments and industry best practices."
                        </p>
                    </div>

                    // Footer Navigation
                    <nav aria-label="Footer Navigation">
                        <ul class="space-y-3">
                            
                        </ul>
                    </nav>

                    // Icon 버튼들
                    <div>
                        <h3 class="text-lg font-semibold mb-4">
                            "Connect With Us"
                        </h3>
                        <div class="flex gap-4">
                        
                        </div>
                    </div>  
                </div>
            </div>
        </footer>
    }
}