use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod components;
pub mod context;
pub mod api;
pub mod lib;
pub mod pages;

use pages::home::Home;
use pages::post::write::PostWrite;
use pages::post_list::PostList;
use pages::post_detail::PostView;
use pages::profile::index::ProfilePage;

use components::theme_switch::ThemeSwitch;
use components::header::Header;
use components::footer::Footer;

use context::theme_context::ThemeContextProvider;
use leptos_toaster::Toaster;

#[tracing::instrument]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/chanism-blog.css"/>
    
        <Script src="https://code.jquery.com/jquery-3.4.1.slim.min.js" integrity="sha384-J6qa4849blE2+poT4WnyKhv5vZF5SrPo0iEjwBvKU7imGFAV0wwj1yYfoRSJoZ+n" crossorigin="anonymous"></Script>
        <Link href="https://cdn.jsdelivr.net/npm/summernote@0.9.0/dist/summernote-lite.min.css" rel="stylesheet" />
        <Script src="https://cdn.jsdelivr.net/npm/summernote@0.9.0/dist/summernote-lite.min.js"></Script>
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/mathjax/3.2.2/es5/tex-mml-chtml.min.js"></Script>
        <Link href="https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.4/codemirror.min.css" rel="stylesheet" />
        <Script src="https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.65.4/codemirror.min.js"></Script>
        <Script src="https://cdn.jsdelivr.net/npm/summernote-codemirror@1.0.0/dist/summernote-codemirror.min.js"></Script>
        // sets the document title
        <Title text="Chanism Blog"/>
        
        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <Toaster
                position=leptos_toaster::ToasterPosition::TopCenter
            >
                <main>
                    <ThemeContextProvider>
                        <Routes>
                            <Route path="" view=HomePage/>
                            <Route path="profile" view=ProfilePage/>
                            <Route path="posts" view=PostPage/>
                            <Route path="posts/:slug" view=PostViewPage/>
                            <Route path="posts/new" view=PostWritePage />
                        </Routes>
                        <ThemeSwitch />
                    </ThemeContextProvider>
                </main>
            </Toaster>
        </Router>
    }
}

#[component]
fn Layout(children : Children) -> impl IntoView {
    view! {
        <main class="min-h-sceen">
            <Header/>
            <div class="mt-9">
                { children() }
            </div>
            <Footer/>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    view! {
        <Layout>
            { 
                || {
                    view! {
                        <Home/>
                    }
                }
            }   
        </Layout>
    }
}

#[component]
fn PostPage() -> impl IntoView {
    view! {
        <Layout>
            {
                || {
                    view! {
                        <PostList />
                    }
                }
            }
        </Layout>
    }
}

#[component]
fn PostViewPage() -> impl IntoView {
    let params = use_params_map();
    let slug = params.with(|p| p.get("slug").cloned()).unwrap_or_else(|| "Unknown".to_string());

    view! {
        <Layout>
            {
                || {
                    view! {
                        <PostView />
                    }
                }
            }
        </Layout>
    }
}

#[component]
fn PostWritePage() -> impl IntoView {
    view! {
        <Layout>
            {
                || {
                    view! {
                        <PostWrite />
                    }
                }
            }
        </Layout>
    }
}