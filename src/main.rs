// The usual CGI stuff.
use cgi::{cgi_try_main, html_response, Request, Response};

// This is a built-in tool that helps us display things when passed into the formatter.
// We're using it in `html_format` so we can avoid doing explicit type conversions.
use std::fmt::Display;

// No changes here. This is still going to be our top-level entrypoint.
cgi_try_main!(exec);

// Oh look! There's a subtle change right here on `exec`! Now we're using
// `anyhow` to help us with our error handle. And you know what the nice thing about this
// is? We don't have to talk about the error handling at all! Because `anyhow` is doing it
// for us. (Well, to be fair, the `cgi_try_main` function is doing a lot here, too.)
fn exec(_request: Request) -> anyhow::Result<Response> {
    let title = "Hello World!";

    // If you get stuck on file loading, try this:
    // let markdown_text = "# Hello there\nIt is so nice to see you".to_string();

    // Now we are going to read the content of the `index.md` file into a string.
    // Remember that `volume` thing we did in `modules.toml`? It mapped all the stuff
    // in our local `content/` directory into the `/` directory. So for us,
    // `content/index.md` is available as `/index.md`.
    let markdown_text = std::fs::read_to_string("/index.md")?;
    // Hey, see that little '?' at the end of the line?                ^^^
    // That tells Rust that if it encounters an error reading the file, it should
    // just bail out of `exec` and return an error to `cgi_try_main`. And then
    // `cgi_try_main` will capture the error, log it, and send back an error message
    // to the browser.

    // Using the `markdown` library we installed, we can render the content of `index.md`
    // into HTML.
    let body = markdown::to_html(&markdown_text);

    // The markdown generator does not do the basic high-level formatting like the body
    // and head tags. Fortunately, we already have our HTML formatter that we can
    // reuse here.
    let doc = html_format(title, &body);

    // And once again, we send the entire thing back to the browser in an HTTP response.
    Ok(html_response(200, doc))
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
