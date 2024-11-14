use leptos::*;

#[derive(Default, Clone, Debug)]
pub enum SectionName {
    #[default] Home,
    About,
    Projects,
    Skills,
    Experience,
    Contact,
}

#[derive(Default, Clone, Debug)]
pub struct SectionState {
    pub active_section: SectionName,
}

pub fn section_name_to_string(section_name: SectionName) -> String {
    match section_name {
        SectionName::Home => "Home".to_string(),
        SectionName::About => "About".to_string(),
        SectionName::Projects => "Projects".to_string(),
        SectionName::Skills => "Skills".to_string(),
        SectionName::Experience => "Experience".to_string(),
        SectionName::Contact => "Contact".to_string(),
    }
}

#[component]
pub fn ActiveSectionContextProvider(
    children: Children
) -> impl IntoView {
    let state = create_rw_signal(SectionState::default());
    provide_context(state);

    view! {
        {
            children()
        }
    }
}