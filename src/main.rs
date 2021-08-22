use cgi::{cgi_try_main, html_response, Request, Response};

// Our shiny new template engine
use handlebars::Handlebars;

// For generating a a serialize and a deserializer
use serde::{Deserialize, Serialize};

const DOC_SEPERATOR: &str = "\n---\n";
const DEFAULT_INDEX: &str = "/index";

cgi_try_main!(exec);

#[derive(Deserialize, Serialize)]
struct Frontmatter {
    title: String,
    description: Option<String>,
}

// This new struct describes all of the stuff that we want to be able to send into
// the handlebars template.
//
// We want all of the stuff from `Frontmatter`. We also want the `body`.
#[derive(Serialize)]
struct TemplateValues {
    frontmatter: Frontmatter,
    body: String,
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
            let (toml_text, markdown_text) = full_document
                .split_once(DOC_SEPERATOR)
                .unwrap_or(("title = 'Untitled'", &full_document));
            let frontmatter: Frontmatter = toml::from_str(toml_text)?;

            // We are going to pass some TemplateValues as structured data to the
            // template engine.
            let values = TemplateValues {
                // We already built the frontmatter
                frontmatter,
                // We do need to render the markdown into HTML, and then set the body
                // to this value.
                body: markdown::to_html(&markdown_text),
            };

            // If the template rendering fails, this will return an Err(),
            // which will result in an 500 error.
            let doc = render_template(values)?;

            // If all goes well, we just send back a response.
            Ok(html_response(200, doc))
        }
        Err(_) => {
            // We should be good citizens and also support templates for errors. That
            // way, if an unfortunate user gets an error, at least it will be a pretty
            // error.
            //
            // Unfortunately, we do have to convert all of our &str to String as we go
            // here. That's why we have all of these `to_string()` calls.
            let values = TemplateValues {
                frontmatter: Frontmatter {
                    title: "Not Found".to_string(),
                    description: Some("Resource was not found on the server".to_string()),
                },
                body: "The requested page was not found.".to_string(),
            };

            // Somewhat ironically, rendering a 404 can cause a 500.
            let body = render_template(values)?;
            Ok(html_response(404, body))
        }
    }
}

// We have replaced html_format() with render_template().
// This new function takes some template values, and tries to render them
// through the MAIN_TEMPLATE, and then return a String.
//
// If it can't load the main template, or if the template can't be rendered,
// this will return an error.
fn render_template(values: TemplateValues) -> anyhow::Result<String> {
    // Handlebars works on JSON data.
    let json_values = serde_json::json!(values);

    // Create a new template engine and render our values into the newly loaded template.
    let mut handlebars = Handlebars::new();
    // Ask Handlebars to load the main template for us
    handlebars.register_template_file("main", "/templates/main.hbs")?;

    // Now we can render a oour values through the template. If this failes, an Err()
    // will be returned, which will result in a 500 Internal Server Error.
    let out = handlebars.render("main", &json_values)?;
    // If it succeeds, we'll send back the output.
    Ok(out)
}
