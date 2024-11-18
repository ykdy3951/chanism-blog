#[allow(unused_imports)]
use leptos::{server, ServerFnError};
#[allow(unused_imports)]
use resend_rs::types::CreateEmailBaseOptions;
#[allow(unused_imports)]
use resend_rs::{Resend, Result};
#[allow(unused_imports)]
use dotenv::dotenv;
#[allow(unused_imports)]
use crate::app::lib::utils::{valid_string, contact_form_email};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SendEmailRequest {
    pub email: String,
    pub message: String,
}

#[server(SendEmail, "/api/email")]
pub async fn send_email(
    req: SendEmailRequest,
) -> Result<(), ServerFnError> {
    dotenv().ok();
    let api_key = std::env::var("RESEND_API_KEY").expect("RESEND_API_KEY must be set");
    let resend = Resend::new(&api_key);
    let reply_to = req.email;
    let message = req.message;

    if !valid_string(&reply_to, 500 as usize) || !valid_string(&message, 5000 as usize) {
        return Err(ServerFnError::ServerError("Invalid email or message".to_string()));
    }

    let from = "Contact Form <onboarding@resend.dev>";
    let to = ["chanism99@gmail.com"];
    let subject = "hello world";
    let html = contact_form_email(reply_to, message);

    let email = CreateEmailBaseOptions::new(from, to, subject)
      .with_html(&html);
  
    let _email = resend.emails.send(email).await?;
  
    Ok(())
}