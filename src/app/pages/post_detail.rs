use leptos::*;
use leptos_icons::*;
use icondata as i;

#[component]
pub fn PostView() -> impl IntoView {
    view! {
        <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <div class="flex flex-col-reverse lg:flex-row gap-12">
                <article class="lg:w-2/3">
                    <PostContents />
                    <CommentView />
                </article>
                <SideBar />
            </div>
        </main>
    }
}

#[component]
pub fn PostContents() -> impl IntoView {
    view! {
        <div class="mb-8">
            <div class="flex items-center gap-4 text-gray-600 text-sm mb-4">
                <time>"Published on November 23, 2024"</time>
                <span>"•"</span>  
                <span>"5 min read"</span>
            </div>

            <h1 class="text-4xl font-bold mb-8">
                "Post Title"
            </h1>

            <img src="https://images.unsplash.com/photo-1517134191118-9d595e4c8c2b?w=1200&auto=format&fit=crop" alt="Post Image" class="w-full h-[400px] mb-8 rounded-lg object-cover"/>

            <div class="prose max-w-none">
                <p>"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."</p>
            </div>
        </div>
    }
}

#[component]
pub fn CommentView() -> impl IntoView {
    view! {
        <section class="mt-16 border-t pt-8">
            <h2 class="text-2xl font-bold mb-8">
                "Comments"
            </h2>

            <div class="mb-8">
                
                <CommentForm />

                // Comment List 
                <div class="space-y-6">
                    {
                        view! {
                            <div class="flex gap-4 items-start">
                                <img 
                                    src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&auto=format&fit=crop"
                                    alt="User avatar"
                                    class="w-10 h-10 rounded-full object-cover"
                                />
                                <div>
                                    <div class="flex items-center gap-2 mb-1">
                                    <span class="font-semibold">Sarah Chen</span>
                                    <span class="text-gray-500 text-sm">2 hours ago</span>
                                    </div>
                                    <p class="text-gray-700">
                                        "Great article! The insights about AI in development are
                                        particularly interesting. I'm curious to see how these
                                        trends will shape the industry in the coming years."
                                    </p>
                                </div>
                            </div>
                        }
                    }
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn CommentForm() -> impl IntoView {

    let (comment_text, set_comment_text) = create_signal("");

    view! {
        <div class="flex gap-4 items-start mb-6">
            <img 
                src="https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?w=100&auto=format&fit=crop"
                alt="User avatar"
                class="w-10 h-10 rounded-full object-cover"
            />
            <div class="flex-1">
                <textarea 
                    placeholder="Add a comment..."
                    bind:value=comment_text
                    class="w-full p-4 border rounded-lg resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
                    rows={3}
                />
                <button class="mt-2 px-4 py-2 bg-blue-600 text-white rounded-lg flex items-center gap-2 hover:bg-blue-700">
                    <Icon icon=i::BsSend />
                    "Post Comment"
                </button>
            </div>
        </div>
    }

}

#[component]
pub fn PostWrite() -> impl IntoView {
    view! {
        <div>
            "PostWrite"
        </div>
    }
}

#[component]
pub fn SideBar() -> impl IntoView {

    let (is_toc_open, set_is_toc_open) = create_signal(false);

    view! {
        <aside class="lg:w-1/3">
          <div class="sticky top-24 border rounded-lg p-6">
            <div class="flex items-center justify-between mb-4">
              <h2 class="text-lg font-semibold">Table of Contents</h2>
              <button 
                on:click=move |_| {
                    set_is_toc_open.set(!is_toc_open.get());
                }
              >
                {
                    move || if is_toc_open.get() {
                        view! {
                            <Icon icon=i::BsChevronUp class="w-5 h-5" />
                        }
                    } else {
                        view! {
                            <Icon icon=i::BsChevronDown class="w-5 h-5" />
                        }
                    }
                }
              </button>
            </div>
            <nav class=move || {if is_toc_open.get() {"space-y-2"} else {"hidden"}}>
                <a
                    href="#emerging-trends"
                    class="block text-gray-600 hover:text-gray-900"
                >
                    "Emerging Trends"
                </a>
                <a
                    href="#ai-development"
                    class="block pl-4 text-gray-600 hover:text-gray-900"
                >
                    "AI in Development"
                </a>
                <a
                    href="#modern-architectures"
                    class="block text-gray-600 hover:text-gray-900"
                >
                    "Modern Architectures"
                </a>
                <a
                    href="#serverless"
                    class="block pl-4 text-gray-600 hover:text-gray-900"
                >
                    "Serverless Computing"
                </a>
            </nav>
          </div>
        </aside>
    }
}