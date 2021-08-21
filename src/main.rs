// Our little program is growing up!
// This time, let's use some stuff from the CGI library.

use cgi::{cgi_try_main, html_response, text_response, Request, Response};

// This is a neat little macro that builds our main function for us, adding some
// error handling. I'm all for using tools that make life easier.
//
// Basically, it will call our `exec` function. If anything goes wrong, it'll send
// back 'ye olde HTTP 500 error.
cgi_try_main!(exec);

// Okay, so this is gonna be our main handler. Every time Wagi gets a new request for
// http://localhost:3000/, this function will handle it.
fn exec(_request: Request) -> Result<Response, String> {
    // This would be equivalent to what we did in the last chapter.
    // Ok(text_response(200, "Hello, CGI!"))

    // But we're gonna do something a little more interesting.
    // Instead of returning a plain text document, we'll send back some HTML.
    let title = "Hello World!";
    let body = "<h1>Hello there</h1><p>It is so nice to see you</p>";
    let doc = html_format(title, body);

    // This cool little html_response function will take our doc, encode it properly,
    // add all the necessary HTTP headers, and send it back to Wagi for us.
    Ok(html_response(200, doc))
}

// This is gonna be our HTML template. It's nothing elaborate.
// It'll basically inject title and body into an HTML shell so we can skip the boilerplate.
fn html_format(title: &str, body: &str) -> String {
    // This is not really the best way of doing template-like things. But it will work
    // for now.
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
