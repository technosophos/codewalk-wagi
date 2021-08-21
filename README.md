# Chapter 6: Handling Errors

In the last chapter, we ended with a server that could determine from the URL which
page to load, and could then load and render the Markdown file.

Accessing `http://localhost:3000/about` would load the `about.md` file. Accessing
`http://localhost:3000/` would match our special index case and load `index.md`.
But what if we were to load `http://localhost:3000/hello`?

Our code would have tried to load that file. But it would fail to find it. But that
would result in the program producing an error and exiting. So the client would get this:

```console
$ curl http://localhost:3000/hello -v
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /hello HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 500 Internal Server Error
< content-length: 0
< date: Sat, 21 Aug 2021 22:16:59 GMT
<
* Connection #0 to host localhost left intact
* Closing connection 0
```

A `500 Internal Server Error` with no useful message. In fact, without the `-v` flag for
`curl`, it would just silently exit.

What we really want to happen is for a miss like that to produce a `404 Not Found` error
with some user-facing text.

That's what this chapter's `src/main.rs` modification is. Go ahead and take a look.

Now go ahead and build and run our new code, and then execute that same `curl` request:

```console
$ curl http://localhost:3000/hello -v
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /hello HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 404 Not Found
< content-length: 209
< content-type: text/html; charset=utf-8
< date: Sat, 21 Aug 2021 22:27:26 GMT
<

        <html>
            <head>
                <title>Not Found</title>
            </head>
            <body>
                The requested page was not found.
            </body>
        </html>
* Connection #0 to host localhost left intact
        * Closing connection 0
```

This time, we correctly get a `404 Not Found` status as well as an HTML page. That's
much better.

We've got just a few more rounds of changes, and then we're done. Next up, let's tackle
fixing the title.

## Take Me Forward! Take Me Back!

Head to chapter 4: `git checkout ch7-frontmatter`
Return to the intro: `git checkout main`
