use leptos::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::window;
use leptos::logging::log;

#[component]
pub fn ScrollingComponent() -> impl IntoView {
    // 상태를 저장하는 시그널을 정의합니다.
    let (scale, set_scale) = create_signal(1.0); // 초기 크기 1.0
    let (opacity, set_opacity) = create_signal(1.0); // 초기 투명도 1.0
    let (style, set_style) = create_signal("".to_string());

    // // 스크롤 이벤트 리스너를 설정합니다.
    // let on_scroll = move |_: web_sys::Event| {
    //     if let Some(scroll_y) = window().and_then(|win| win.scroll_y().ok()) {
    //         let scroll_y = scroll_y as f64;

    //         log!("scroll_y: {}", scroll_y);

    //         let new_scale = 1.0 - (scroll_y * 0.001).min(0.5); // 최대 축소 0.5
    //         let new_opacity = 1.0 - (scroll_y * 0.001).min(0.5); // 최대 투명도 0.5

    //         set_scale.update(|scale | *scale = new_scale);
    //         set_opacity.update(|opacity | *opacity = new_opacity);
    //         set_style.update(|style| *style = format!("transform: scale({}); opacity: {}", new_scale, new_opacity));
    //     }
    // };

    let on_scroll = move |_: web_sys::Event| {
        if let Some(window) = window() {
            let scroll_y = window.scroll_y().unwrap() as f64;

            let progress_ratio = scroll_y / window.inner_height().unwrap().as_f64().unwrap().max(scroll_y);
            let new_scale = 0.8 + (1.0 - 0.8) * progress_ratio;
            let new_opacity = 0.6 + (1.0 - 0.6) * progress_ratio;

            log!("scroll_y: {} inner_height : {}", scroll_y, window.inner_height().unwrap().as_f64().unwrap());

            set_scale.update(|scale| *scale = new_scale);
            set_opacity.update(|opacity| *opacity = new_opacity);
            set_style.update(|style| *style = format!("transform: scale({}); opacity: {}", new_scale, new_opacity));
        }
    };

    create_effect(move |_| {
        if let Some(window) = window() {
            let closure = Closure::wrap(Box::new(on_scroll.clone()) as Box<dyn FnMut(_)>);
            window
                .add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();
        }
    });

    // 컴포넌트 렌더링
    view! { 
        <main class="flex h-[8000px] flex-col-reverse">
            <div
                style={style.clone()}
                class="transition-all duration-300 w-64 h-64 bg-blue-500 fixed top-20 left-20"
            >
            {move || scale.get().to_string() + " " + &opacity.get().to_string()}
            </div>
        </main>
    }
}