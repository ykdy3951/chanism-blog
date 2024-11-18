pub fn class_names (
    base_classes: &[&str],
    conditional_classes: &[(&str, bool)],
) -> String {
    let mut class_names = Vec::new();

    for base_class in base_classes {
        if base_class.is_empty() {
            continue;
        }
        class_names.push(base_class.to_string());
    }

    for (class_name, condition) in conditional_classes {
        if *condition {
            class_names.push(class_name.to_string());
        }
    }

    class_names.join(" ")
}

pub fn valid_string (string: &str, max_length: usize) -> bool {
    string.len() <= max_length
}

pub fn contact_form_email(to : String, message : String) -> String {
    let styles = r#"
        html {
            font-size: 16px; /* 기준 폰트 크기 */
        }
        body {
            font-family: 'Arial', sans-serif;
            background-color: #f3f4f6;
            margin: 0;
            padding: 0;
            line-height: 1.5em;
            color: #374151;
            -webkit-font-smoothing: antialiased;
        }
        .container {
            max-width: 600px;
            margin: 20px auto;
            background: #ffffff;
            border-radius: 0.75rem; /* 12px */
            overflow: hidden;
            box-shadow: 0 0.25rem 0.75rem rgba(0, 0, 0, 0.1); /* 4px 12px */
            border: 1px solid #e5e7eb;
        }
        .header {
            background-color: #4f46e5;
            color: #ffffff;
            text-align: center;
            padding: 1.25rem; /* 20px */
            font-size: 1.5rem; /* 24px */
            font-weight: bold;
        }
        .content {
            padding: 1.25rem; /* 20px */
            font-size: 1rem; /* 16px */
            color: #374151;
        }
        .content p {
            margin: 0 0 1rem; /* 16px */
            font-size: 1rem; /* 16px */
        }
        .button {
            display: inline-block;
            padding: 0.75rem 1.5rem; /* 12px 24px */
            margin-top: 1rem; /* 16px */
            background-color: #4f46e5;
            color: #ffffff;
            text-decoration: none;
            font-weight: bold;
            font-size: 1rem; /* 16px */
            border-radius: 0.375rem; /* 6px */
            text-align: center;
        }
        .footer {
            background-color: #f9fafb;
            text-align: center;
            padding: 0.9375rem; /* 15px */
            font-size: 0.75rem; /* 12px */
            color: #6b7280;
        }
        @media (max-width: 600px) {
            html {
                font-size: 14px; /* 모바일 화면에서 기준 크기 축소 */
            }
            .container {
                margin: 10px;
                width: 100%;
                box-shadow: none;
                border-radius: 0.5rem; /* 8px */
            }
            .content {
                padding: 1rem; /* 16px */
            }
            .button {
                padding: 0.625rem 1.25rem; /* 10px 20px */
                font-size: 0.875rem; /* 14px */
            }
            a:link {
                text-decoration: none;
            }
        }
    "#;

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>{styles}</style>
        </head>
        <title>New message from your website</title>
        <body>
            <div class="container">
                <div class="header">
                    {subject}
                </div>
                <div class="content">
                    <p>Hello,</p>
                    <p>You have received a new message from the contact form:</p>
                    <p><strong>Message:</strong></p>
                    <p>{message}</p>
                    <a href="mailto:{to}" class="button" style="color:#FFFFFF; text-decoration:none;">Reply to {to}</a>
                </div>
                <div class="footer">
                    &copy; 2024 Your Company. All rights reserved.
                </div>
            </div>
        </body>
        </html>
        "#,
        styles = styles,
        subject = "New message from contact form",
        message = message,
        to = to,
    )
}