use leptos::*;
use leptos_icons::*;
use icondata as i;
use leptos_router::*;
use slug::slugify;

#[component]
pub fn PostList() -> impl IntoView {
    view! {
        <section class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <div class="flex flex-col sm:flex-row justify-between items-stretch sm:items-center gap-4 mb-8">
                <select class="px-4 py-2 border border-gray-200 rounded-lg bg-white">
                    <option>All Topics</option>
                    <option>Technology</option>
                    <option>Design</option>
                    <option>Development</option>
                </select>

                <div class="relative flex-1">
                    <Icon icon=i::FiSearch class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
                    <input
                        type="search"
                        placeholder="Search articles..."
                        class="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg"
                    />
                </div>
            </div>

            <div class="flex flex-col gap-8">
                <PostCard/> 
                <PostCard/> 
                <PostCard/> 

            </div>

            <div class="flex justify-center items-center space-x-2 mt-12">
                <button class="p-2 rounded-lg border border-gray-200 hover:bg-gray-50">
                <Icon icon=i::BsChevronLeft class="w-5 h-5" />
                </button>
                <button class="px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50">
                1
                </button>
                <button class="px-4 py-2 rounded-lg bg-blue-600 text-white">
                2
                </button>
                <button class="px-4 py-2 rounded-lg border border-gray-200 hover:bg-gray-50">
                3
                </button>
                <button class="p-2 rounded-lg border border-gray-200 hover:bg-gray-50">
                <Icon icon=i::BsChevronRight class="w-5 h-5" />
                </button>
            </div>
        </section>
    }
}

#[component]
fn PostCard() -> impl IntoView {
    view! {
        <article class="flex flex-col md:flex-row rounded-lg shadow-sm hover:shadow-lg transition-shadow duration-200 overflow-hidden border border-gray-200">
          <div class="flex-1 md:w-[60%] p-6">
            <div class="flex items-center space-x-2 mb-4">
              <span class="px-3 py-1 text-sm rounded-full bg-blue-50 text-blue-600">
                Technology
              </span>
              <span class="px-3 py-1 text-sm rounded-full bg-purple-50 text-purple-600">
                AI
              </span>
            </div>

            <h2 class="text-xl font-bold mb-2">
              The Future of Artificial Intelligence in Modern Software
              Development
            </h2>

            <p class="text-gray-600 mb-4">
              Exploring how AI is revolutionizing the way we build and maintain
              software applications in the modern development landscape...
            </p>

            <div class="flex items-center justify-between mt-auto">
              <div class="flex items-center text-sm text-gray-500">
                <Icon icon=i::BsClock class="w-4 h-4 mr-1" />
                <span>8 min read</span>
                <span class="mx-2">"•"</span>
                <span>March 15, 2024</span>
              </div>
              <button class="flex items-center text-blue-600 hover:text-blue-700 transition-colors duration-200"
                on:click={
                    move |_| {
                        let navigate = use_navigate();
                        let slug = slugify("The Future of Artificial Intelligence in Modern Software Development");
                        navigate(format!("/posts/{}", slug).as_str(), Default::default());
                    }
                }
              >
                Read Story
                <Icon icon=i::FiArrowRight class="w-4 h-4 ml-1" />
              </button>
            </div>
          </div>

          <div class="relative h-48 md:h-auto md:w-[40%] max-md:hidden">
            <img
              src="https://images.unsplash.com/photo-1485827404703-89b55fcc595e"
              alt="AI Technology"
              class="absolute inset-0 w-full h-full object-cover"
            />
          </div>
        </article>
    }
}