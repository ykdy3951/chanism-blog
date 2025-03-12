use leptos::*;
use leptos_icons::*;
use icondata as i;
use leptos_router::use_navigate;

#[component]
pub fn Recommendations() -> impl IntoView {

    let (current_slide, set_current_slide) = create_signal(0);

    let slide_style = move || format!("transform: translateX(-{}%);", current_slide() * 100);

    let next_slide = move || set_current_slide.update(|current| *current = (&*current + 1) % 3);
    let prev_slide = move || set_current_slide.update(|current| *current = (&*current - 1) % 3);

    view! {
        <section class="relative h-[500px] overflow-hidden">
            <div class="flex transition-transform duration-500 h-full"
                style=slide_style
            >
                <div class="min-w-full h-full relative">
                    <img src="https://picsum.photos/1024/768" alt="Slide 1" class="w-full h-full object-cover"/>
                    <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black to-transparent p-8">
                        <div class="max-w-7xl mx-auto">
                            <h2 class="text-white text-4xl font-bold mb-4">
                                "Slide 1"
                            </h2>
                            <p class="text-lg text-gray-200">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                            </p>
                        </div>
                    </div>
                </div>
                <div class="min-w-full h-full relative">
                    <img src="https://picsum.photos/1024/768" alt="Slide 2" class="w-full h-full object-cover"/>
                    <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black to-transparent p-8">
                        <div class="max-w-7xl mx-auto">
                            <h2 class="text-white text-4xl font-bold mb-4">
                                "Slide 2"
                            </h2>
                            <p class="text-lg text-gray-200">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                            </p>
                        </div>
                    </div>
                </div>
                <div class="min-w-full h-full relative">
                    <img src="https://picsum.photos/1024/768" alt="Slide 3" class="w-full h-full object-cover"/>
                    <div class="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black to-transparent p-8">
                        <div class="max-w-7xl mx-auto">
                            <h2 class="text-white text-4xl font-bold mb-4">
                                "Slide 3"
                            </h2>
                            <p class="text-lg text-gray-200">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            <button class="absolute left-4 top-1/2 -translate-y-1/2 p-2 rounded-full bg-white/80 hover:bg-white transition-colors"
                on:click=move |_| prev_slide()
            >
                <Icon icon=i::FaChevronLeftSolid class="w-4 h-4"/>
            </button>
            <button class="absolute right-4 top-1/2 -translate-y-1/2 p-2 rounded-full bg-white/80 hover:bg-white transition-colors"
                on:click=move |_| next_slide()
            >
                <Icon icon=i::FaChevronRightSolid class="w-4 h-4"/>
            </button>
        </section>
    }
}

#[component]
pub fn TagSection() -> impl IntoView {
    view! {
        <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <div key="AI" class="mb-12">
                <h2 class="text-2xl font-bold mb-6">
                    "AI"
                </h2>
                <div class="flex gap-6 overflow-x-auto pb-4 scrollbar-hide">
                    <div key="Post 1" class="min-w-[300px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <img src="https://picsum.photos/300/200" alt="Post 1" class="w-full h-48 object-cover"/>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-2">
                                "Post 1"
                            </h3>
                            <p class="text-gray-600">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            </p>
                        </div>
                    </div>
                    <div key="Post 2" class="min-w-[300px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <img src="https://picsum.photos/300/200" alt="Post 2" class="w-full h-48 object-cover"/>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-2">
                                "Post 2"
                            </h3>
                            <p class="text-gray-600">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            </p>
                        </div>
                    </div>
                    <div key="Post 3" class="min-w-[300px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <img src="https://picsum.photos/300/200" alt="Post 3" class="w-full h-48 object-cover"/>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-2">
                                "Post 3"
                            </h3>
                            <p class="text-gray-600">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            </p>
                        </div>
                    </div>
                    <div key="Post 4" class="min-w-[300px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <img src="https://picsum.photos/300/200" alt="Post 4" class="w-full h-48 object-cover"/>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-2">
                                "Post 4"
                            </h3>
                            <p class="text-gray-600">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            </p>
                        </div>
                    </div>
                    <div key="Post 5" class="min-w-[300px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                        <img src="https://picsum.photos/300/200" alt="Post 5" class="w-full h-48 object-cover"/>
                        <div class="p-4">
                            <h3 class="text-lg font-bold mb-2">
                                "Post 5"
                            </h3>
                            <p class="text-gray-600">
                                "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

// 3개의 최근 포스트를 보여주는 컴포넌트
#[component]
pub fn RecentPosts() -> impl IntoView {

    view! {
        <section class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <h2 class="text-2xl font-bold mb-6">
                "Recent Posts"
            </h2>
            <div class="flex gap-6 pb-4">
                <div class="min-w-[350px] bg-white rounded-lg overflow-hidden shadow-sm hover:shadow-md transition-shadow cursor-pointer">
                    <img src="https://picsum.photos/300/200" alt="Post 1" class="w-full h-48 object-cover"/>
                    <div class="p-4">
                        <h3 class="text-lg font-bold mb-2">
                            "Post 1"
                        </h3>
                        <p class="text-gray-600">
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
                        </p>
                    </div>
                </div>
            </div>
            <div class="text-center mt-8">
                <button class="inline-flex items-center px-6 py-3 bg-gray-100 hover:bg-gray-200 transition-colors rounded-lg font-bold"
                    on:click=move |_| {
                        let navigate = use_navigate();
                        navigate("/posts", Default::default());
                    }
                >
                    "Explore All Posts"
                    <Icon icon=i::FaChevronRightSolid class="ml-2 w-5 h-5"/>
                </button>
            </div>
        </section>
    }
}


#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Recommendations/>
        <TagSection/>
        <RecentPosts/>
    }
}