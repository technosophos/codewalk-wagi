use cgi::{cgi_try_main, html_response, Request, Response};
use std::fmt::Display;

// For generating a deserializer
use serde::Deserialize;

// This is what separates our frontmatter from our Markdown.
const DOC_SEPERATOR: &str = "\n---\n";
const DEFAULT_INDEX: &str = "/index";

cgi_try_main!(exec);

// This struct represents our frontmatter. It is decorated with `#[derive(Deserialize)]`
// to tell the Rust compiler that we want to deserialze our TOML frontmatter. The compiler
// then automatically generates the deserializer for us.
#[derive(Deserialize)]
struct Frontmatter {
    // Title is required. How do we know? It's type is `String`.
    title: String,
    // What if we wanted to add a description field? It would look something
    // like this. To make it optional, we use `Option<String>`.
    description: Option<String>,
}

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

    match std::fs::read_to_string(format!("{}.md", path_info)) {
        Ok(full_document) => {
            // This is going to take `full_document`, split it into two objects at
            // the `---`, and assign those to `toml_text` and `markdown_text`.
            // But wait... what if we have old Markdown that doesn't have
            // any frontmatter? Well then... we just fake some TOML data and
            // pass the entire document into the `markdown_text`.
            let (toml_text, markdown_text) = full_document
                .split_once(DOC_SEPERATOR)
                .unwrap_or(("title = 'Untitled'", &full_document));

            // Now we need to parse the TOML into a `Frontmatter` object.
            // If the TOML document is malformed, this will send an Err()
            // result, which will result in a 500 error. That is probably the right
            // thing to do in this case.
            let frontmatter: Frontmatter = toml::from_str(toml_text)?;

            // Set the title using the value in the frontmatter.
            let title = frontmatter.title;

            // From here on out, everything else is the same!
            let body = markdown::to_html(&markdown_text);
            let doc = html_format(title, body);
            Ok(html_response(200, doc))
        }
        Err(_) => {
            let body = html_format("Not Found", "The requested page was not found.");
            Ok(html_response(404, body))
        }
    }
}

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
