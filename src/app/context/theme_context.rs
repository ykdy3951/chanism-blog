use leptos::*;
use leptos::logging::log;
#[derive(Default, Clone, Debug)]
pub enum ThemeName {
    #[default] Light,
    Dark,
}

#[derive(Default, Clone, Debug)]
pub struct ThemeState {
    pub theme: ThemeName,
}

#[component]
pub fn ThemeContextProvider(
    children: Children
) -> impl IntoView {
    let state = create_rw_signal(ThemeState::default());
    provide_context(state);
    
    create_effect(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        match state.get().theme {
            ThemeName::Dark => {
                if let Some(local_storage) = window.local_storage().unwrap() {
                    local_storage.set_item("theme", "dark").unwrap();
                }
                window.document().unwrap().document_element().unwrap().class_list().add_1("dark").unwrap();
            }
            ThemeName::Light => {
                if let Some(local_storage) = window.local_storage().unwrap() {
                    local_storage.set_item("theme", "light").unwrap();
                }
                window.document().unwrap().document_element().unwrap().class_list().remove_1("dark").unwrap();
            }
        }
    });

    create_effect(move |_| {
        let window = web_sys::window().expect("no global `window` exists");
        log!("aaa");
        if let Some(local_storage) = window.local_storage().unwrap() {
            if let Ok(Some(theme)) = local_storage.get_item("theme") {
                match theme.as_str() {
                    "dark" => {
                        state.set(ThemeState { theme: ThemeName::Dark });
                        window.document().unwrap().document_element().unwrap().class_list().add_1("dark").unwrap();
                    }
                    _ => {
                        state.set(ThemeState { theme: ThemeName::Light });
                        window.document().unwrap().document_element().unwrap().class_list().remove_1("dark").unwrap();
                    }
                }
            }
        }

        else if web_sys::window().unwrap().match_media("(prefers-color-scheme: dark)").unwrap().expect("REASON").matches() {
            log!("dark");
            state.set(ThemeState { theme: ThemeName::Dark });
            window.document().unwrap().document_element().unwrap().class_list().add_1("dark").unwrap();
        }
    });

    view! {
        {
            children()
        }
    }
}