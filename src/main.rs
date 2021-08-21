use cgi::{cgi_try_main, html_response, Request, Response};
use std::fmt::Display;

// If we get a request for "/", which file should we load? The value of this
// string will be used to determine that.
const DEFAULT_INDEX: &str = "/index";

cgi_try_main!(exec);

fn exec(request: Request) -> anyhow::Result<Response> {
    //  ^ We removed an underscore from _right there_. This is because Rust uses
    // underscores to tell the compiler "I'm intentionally not using this variable".
    // And before we were not using `_request`. But now we are. So we rename it to
    // just plain old `request`.

    // The first thing we need to do is find out what file was requested.
    // There are a few ways we could get this information... but the easiest one is to
    // look at the `X-CGI-Path-Info` header that is set by the `cgi` module we are using:
    let path_info = match request.headers().get("X-CGI-Path-Info") {
        // If you are new to Rust, this part might feel weird. Here's what's going on:
        // The `match` statement works kind of like a switch statement in other languages.
        // But we can do some more sophisticated things with it. In this case, we are
        // saying:
        // - If our call to get() returns some header value, then that's our path.
        // - If get() did not find an X-CGI-Path-Info value, then we'll go with the
        //   default index. (Because the only case where Path Info is not set is
        //   if the route in `modules.toml` is `/` instead of `/...`)
        Some(header) => {
            let path = header.to_str()?;

            // We have one special case to handle: If the URL goes to the root, we need
            // to know which file to load. In that case, we use DEFAULT_INDEX, which
            // we defined at the top of this file.
            if path == "/" {
                DEFAULT_INDEX
            } else {
                path
            }
        }
        None => "/index",
    };

    // This is the easiest way to log a message to our error log:
    eprintln!("Access {}", path_info);

    // So what we're going to do is convert from the inbound path to a file by the same
    // name that ends with `.md`. So `/index` becomes `/index.md`. Then we look up
    // on the filesystem. Now, the only files on Wagi's filesystem are the ones in our
    // volume, which means only the stuff in `content/`. So that makes this method
    // relatively safe. Nobody can break out of the path and somehow access other files.
    let markdown_text = std::fs::read_to_string(format!("{}.md", path_info))?;

    // The rest of this is the same as the previous version.
    let title = "Hello World!";
    let body = markdown::to_html(&markdown_text);
    let doc = html_format(title, &body);
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
