# Chapter 3: Hello CGI

At first glance, not much has changed in our project. In fact, we haven't added any
new files. But we did change some code.

The first thing to notice is that we added a new dependency in `Cargo.toml`. It's called
`cgi`, and it adds support for converting Wagi's CGI formatted data to a simple
HTTP Request/Response model.

You can head on over to `src/main.rs` and see how we used it.

The important thing to gain from this example is that we have switched from doing
raw CGI to using a lightweight framework. While this initially makes our code a little
bit longer, it means we won't in the future, have to spend our time parsing out lots
of environment variables or manually serializing data.

So that's it for this chapter. Next up, we'll add Markdown handling and learn a little
more about Wagi's features.

## Take Me Forward! Take Me Back!

Head to chapter 4: `git checkout ch4-markdown`
Return to the intro: `git checkout main`
