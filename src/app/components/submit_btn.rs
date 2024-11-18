use leptos::*;
use leptos_icons::*;
use icondata as i;

#[component]
pub fn SubmitBtn(
    is_pending: ReadSignal<bool>
) -> impl IntoView {

    let button_content = create_signal(
        move || {
            if is_pending() {
                view! {
                    <div class="h-5 w-5 animate-spin rounded-full border-b-2 border-white"></div>
                }
            }
            else {
                view! { 
                    <div class="flex w-full h-full gap-2 items-center justify-center">
                        "Submit" 
                        <Icon 
                            icon={i::FaPaperPlaneSolid} 
                            class="text-xs opacity-70 transition-all group-hover:translate-x-1 group-hover:-translate-y-1"
                        />
                        " "
                    </div>
                }
            }
        }
    );

    view! {
        <button 
            type="submit" 
            class="group flex items-center justify-center gap-2 h-[3rem] w-[8rem] bg-gray-900 text-white rounded-full outline-none transition-all focus:scale-110 hover:scale-110 active:scale-105 hover:bg-gray-950 dark:bg-white dark:bg-opacity-10 disabled:scale-100 disabled:bg-opacity-65"
            disabled=is_pending
        >
            {button_content.0()}
        </button>
    }
}