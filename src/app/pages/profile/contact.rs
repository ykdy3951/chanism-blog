use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::FormData;
use crate::app::pages::profile::submit_btn::SubmitBtn;
use crate::app::pages::profile::section_heading::SectionHeading;
use crate::app::lib::hooks::use_section_in_view;
use crate::app::context::active_section_context::{SectionState, SectionName};
use crate::app::api::email::{send_email, SendEmailRequest};
use leptos_toaster::*;
use leptos::logging::log;

#[component]
pub fn Contact() -> impl IntoView {

    let is_visible = use_section_in_view("contact", 0.75, 1000.0);
    let (is_pending, set_is_pending) = create_signal(false);
    let toast_context = expect_context::<Toasts>();

    let send_email_action = create_action(move |form_data: &FormData| {
        let form_data = form_data.clone();
        set_is_pending.set(true);

        async move {
            let email = form_data.get("email").as_string().expect("email not found");
            let message = form_data.get("message").as_string().expect("message not found");
            let form_data = SendEmailRequest { email, message };
            let result = send_email(form_data).await;

            set_is_pending.set(false);
            result
        }
    });

    let create_toast = move || {
        let toast_id = ToastId::new();

        log!("send_email_action: {:?}", send_email_action.value().get());

        let options = match send_email_action.value().get() {
            Some(Ok(_)) => (ToastVariant::Success, "Email sent successfully!"),
            Some(Err(_)) => (ToastVariant::Error, "Failed to send email!"),
            None => (ToastVariant::Normal, "Default toast")
        };
        
        toast_context.toast(
            
            view! { 
                <Toast 
                    toast_id 
                    variant=options.0
                    title=view! { <span>{options.1}</span> }.into_view()
                    
			    /> 
            },
            Some(toast_id),
            None // options
        );
    };


    create_effect(move |_| {
        if send_email_action.value().get().is_some() {
            create_toast();
        }
    });
    

    create_effect(move |_| {
        if is_visible.get() {
            let state = use_context::<RwSignal<SectionState>>().expect("ActiveSectionContextProvider not found");
            state.update(|state| state.active_section = SectionName::Contact);
        }
    });

    view! {
        <section id="contact" class="mb-20 sm:mb-28 w-2/3 max-w-[42rem] text-center">
            <SectionHeading>
                "Contact Me"
            </SectionHeading>
            <p
                class="text-gray-700 dark:text-white/80"
            >
                "Please contact me direct at "
                <a href="mailto:chanism99@gmail.com" class="text-blue-500 underline">
                    "chanism99@gmail.com"
                </a>
                " or use the form below."
            </p>
            <form 
                class="mt-10 flex flex-col dark:text-black"
                on:submit=move |event| {
                    event.prevent_default();
                    let form_data = FormData::new_with_form(event.target().unwrap().unchecked_ref()).unwrap();
                    send_email_action.dispatch(form_data);
                }
            >
                <input 
                    type="email" 
                    name="email"
                    class="h-14 rounded-lg border border-black/10 p-4 dark:bg-white dark:bg-opacity-80 dark:focus:bg-opacity-100 transition-all dark:outline-none"
                    placeholder="Your email" 
                    required
                    maxlength=100
                />
                <textarea 
                    placeholder="Your message" 
                    name="message"
                    class="h-52 my-3 rounded-lg border border-black/10 p-4 dark:bg-white dark:bg-opacity-80 dark:focus:bg-opacity-100 transition-all dark:outline-none"
                    required
                    maxlength=5000
                />
                <SubmitBtn 
                    is_pending=is_pending
                />
            </form>
        </section>
    }
}