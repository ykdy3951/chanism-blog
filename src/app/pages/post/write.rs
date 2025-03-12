use ev::KeyboardEvent;
use leptos::*;
use leptos::logging::log;
use leptos_icons::*;
use icondata as i;
use wasm_bindgen::JsCast;
use gloo::file::File;
use reqwest::multipart::{Form, Part};

#[component]
pub fn PostWrite() -> impl IntoView {
    
    let (title, set_title) = create_signal(String::new());
    let (tags, set_tags) = create_signal(vec![]);
    let (tag_input, set_tag_input) = create_signal(String::new());
    let (file_name, set_file_name) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());

    let handle_tag_add = move |event: KeyboardEvent| {
        if event.key() == "Enter" {
            let tag = tag_input.get().trim().to_string();
            if !tag.is_empty() && !tags.get().contains(&tag) {
                set_tags.update(|t| t.push(tag.clone()));
            }
            set_tag_input("".to_string());
            event.prevent_default();
        }
    };

    let remove_tag = move |tag: String| {
        set_tags.update(|t| t.retain(|t| t != &tag));
        log!("{:?}", tags.get());
    };

    let handle_file_change = move |event: web_sys::Event| {
        if let Some(input) = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().ok() {
            if let Some(files) = input.files() {
                if let Some(file) = files.get(0) {
                    let file_name = file.name();
                    let file_blob = gloo::file::Blob::from(file);
                    
                    spawn_local(async move {
                        let file_data = gloo::file::futures::read_as_bytes(&file_blob)
                            .await
                            .expect("Failed to read file");
                        
                        let part = Part::bytes(file_data)
                            .file_name(file_name);
    
                        let form = Form::new().part("file", part);
                        
                        match reqwest::Client::new()
                            .post("http://localhost:3000/api/uploads")
                            .multipart(form)
                            .send()
                            .await
                        {
                            Ok(response) if response.status().is_success() => {
                                web_sys::console::log_1(&"File uploaded successfully".into());
                            }
                            Ok(response) => {
                                web_sys::console::log_1(
                                    &format!("Failed to upload file: {:?}", response).into(),
                                );
                            }
                            Err(err) => {
                                web_sys::console::log_1(
                                    &format!("Error during file upload: {:?}", err).into(),
                                );
                            }
                        }
                    });
                }
            }
        }
    };

    view! {
        <section class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
            <h1 class="text-2xl font-bold mb-8">
                "Write a Post"
            </h1>
            <div class="space-y-6">
                <div class="space-y-2">
                    <label htmlFor="title" class="block text-lg font-medium">
                        "Title"
                    </label>
                    <input
                        id="title"
                        type="text"
                        value={title}
                        on:change=move |event| {
                            set_title(event_target_value(&event));
                        }
                        placeholder="Enter your blog post title..."
                        class="w-full px-4 py-3 text-lg border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-shadow"
                        aria-label="Blog post title"
                    />
                </div>
                <div class="space-y-2">
                    <label htmlFor="cover_image" class="block text-lg font-medium">
                        "Cover Image"
                    </label>
                    <input
                        id="cover_image"
                        type="file"
                        class="w-full px-4 py-3 text-lg border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-shadow"
                        aria-label="Cover image"
                        on:change=handle_file_change
                    />
                </div>
                <div class="space-y-2">
                    <label htmlFor="content" class="block text-lg font-medium">
                        Content
                    </label>
                    <div id="summernote"></div>
                    <script>
                        $("#summernote").summernote({
                            placeholder: "Hello stand alone ui",
                            tabsize: 2,
                            height: 300,
                            codemirror: {
                                theme: "monokai", // 선택 가능한 테마
                                lineNumbers: true,
                            },
                            toolbar: [
                            ["style", ["style"]],
                            ["font", ["bold", "italic", "underline", "clear"]],
                            ["color", ["color"]],
                            ["para", ["ul", "ol", "paragraph"]],
                            ["table", ["table"]],
                            ["insert", ["link", "picture", "video"]],
                            ["view", ["fullscreen", "codeview", "help"]]
                            ],
                        });
                    </script>
                </div>
                <div class="space-y-2">
                    <label htmlFor="tags" class="block text-lg font-medium">
                        Tags
                    </label>
                    <div class="space-y-3">
                        <div class="flex flex-wrap gap-2">
                            {
                                move || {
                                    let tags = tags.get();
                                    tags.into_iter().map(|tag| {
                                        let copy_tag = String::from(tag.clone());
                                        view! {
                                            <span class="inline-flex items-center px-3 py-1 rounded-full bg-gray-100 text-sm border-gray-200">
                                                <Icon icon=i::BiHashRegular class="w-4 h-4 mr-1" />
                                                {copy_tag.clone()}
                                                <button
                                                    class="ml-2 hover:text-gray-700"
                                                    aria-label=format!("Remove tag: {}", copy_tag)
                                                    on:click=move |_| {
                                                        remove_tag(tag.clone());
                                                    }
                                                >
                                                    <Icon icon=i::RiCloseSystemFill class="w-4 h-4" />
                                                </button>
                                            </span>
                                        }
                                    }).collect::<Vec<_>>()
                                }
                            }
                        </div>
                        <input
                            id="tag-input"
                            type="text"
                            value={tag_input}
                            on:input=move |e| {
                                set_tag_input(event_target_value(&e));
                            }
                            on:keydown=handle_tag_add
                            placeholder="Add a tag..."
                            class="w-full px-4 py-2 border rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none"
                            aria-label="Add a tag"
                        />
                    </div>
                </div>
            </div>
            <button type="submit" class="mt-4 px-4 py-2 bg-blue-500 text-white rounded"
                on:click=move |_| {
                    let content = document().get_element_by_id("summernote").unwrap().inner_html();
                    log!("{}", content);
                }
            >
                "Submit"
            </button>
        </section>
    }
}