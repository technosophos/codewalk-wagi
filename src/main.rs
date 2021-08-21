use cgi::{cgi_try_main, html_response, Request, Response};
use std::fmt::Display;

// If we get a request for "/", which file should we load? The value of this
// string will be used to determine that.
const DEFAULT_INDEX: &str = "/index";

cgi_try_main!(exec);

fn exec(request: Request) -> anyhow::Result<Response> {
    // This stuff up here is all the same as last time.
    let path_info = match request.headers().get("X-CGI-Path-Info") {
        Some(header) => {
            let path = header.to_str()?;
            if path == "/" {
                DEFAULT_INDEX
            } else {
                path
            }
        }
        None => "/index",
    };

    eprintln!("Access {}", path_info);

    // The original 404 problem happened because `read_to_string` could not find a file
    // to load. So let's refactor some code to say:
    //   - If read_to_string gets some data, format it and send it back
    //   - If it fails, send a 404 error with a helpful message.
    //
    // Because it's so useful, we're going to use another `match` here.
    match std::fs::read_to_string(format!("{}.md", path_info)) {
        Ok(markdown_text) => {
            // This is basically the same stuff as before.
            let title = "Hello World!";
            let body = markdown::to_html(&markdown_text);
            let doc = html_format(title, &body);
            Ok(html_response(200, doc))
        }
        Err(_) => {
            // If there is an error reading the file, write back a 404 message.
            let body = html_format("Not Found", "The requested page was not found.");
            // Ok() means there was no error during processing, right? So why is this returning
            // an Ok() instead of an Err()? Because the `Ok()` merely informs the CGI runner
            // that we have already handled the error. If we returned `Err()`, it would
            // signal the CGI runner that IT needed to handle the error. And it would do
            // so by returning a 500.
            Ok(html_response(404, body))
        }
    }
}

// We used this function last time. And it's useful again here. We changed the function
// signature to make it a little more generic. Specifically, we changed it to say that
// it doesn't matter what type `title` and `body` are as long as they can be automatically
// converted into a something we can display. This saves us from having to do manual conversions.
fn html_format<I: Display>(title: I, body: I) -> String {
    format!(
        r#"
        <html>
            <head>
                <title>{}</title>
            </head>
            <body>
                {}
            </body>
        </html>
        "#,
        title, body
    )
}
