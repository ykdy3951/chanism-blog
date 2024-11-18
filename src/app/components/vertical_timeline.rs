use leptos::*;
use crate::app::lib::utils::class_names;

#[derive(Debug, Clone, Default)]
pub enum VerticalTimelineLayout {
    #[default] TwoColumns,
    OneColumn,
    OneColumnLeft,
    OneColumnRight,
}

#[component]
pub fn VerticalTimeline(
    children: Children,
    #[prop(default = true)]
    animate: bool,
    #[prop(default = VerticalTimelineLayout::TwoColumns)]
    layout: VerticalTimelineLayout,
    #[prop(default = "#FFF".to_string())]
    line_color: String,
    #[prop(default = "".to_string())]
    class: String,
) -> impl IntoView{

    let layout_class = match layout {
        VerticalTimelineLayout::TwoColumns => "vertical-timeline--two-columns",
        VerticalTimelineLayout::OneColumn => "vertical-timeline--one-column-left",
        VerticalTimelineLayout::OneColumnLeft => "vertical-timeline--one-column-left",
        VerticalTimelineLayout::OneColumnRight => "vertical-timeline--one-column-right",
    };
    
    create_effect(move |_| {
        if line_color.is_empty() {
            return;
        }

        let document = web_sys::window().unwrap().document().unwrap();
        if let Some(html_element) = document.document_element() {
            html_element.set_attribute("style", &format!("--line-color: {}", line_color)).unwrap();
        }
    });

    view! {
        <div 
            class=move || 
            {
                class_names(&["vertical-timeline", layout_class, class.as_str()], &[("vertical-timeline--animate", animate)])
            }
        >
            {
                children()
            }
        </div>
    }
}