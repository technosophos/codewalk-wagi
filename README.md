# Chapter 5: Multi-page Server

Let's kick it up a notch! In this chapter we will extend our app to serve multiple pages
and then see how we can link between the pages.

## A quick word on errors

Maybe it's too late for this... maybe you've already hit a snag or two. But sometimes 
as you're coding away, you'll get a big ugly `500` error (or nothing at all) when you
load a page. Or maybe your Wagi program will print out something like this:

```
Aug 21 13:50:28.994 ERROR wagi::runtime: error running WASM module error=wasm trap: unreachable
wasm backtrace:
    0: 0x2de327 - <unknown>!__rust_start_panic
    1: 0x2ddfde - <unknown>!rust_panic
    2: 0x2ddb74 - <unknown>!std::panicking::rust_panic_with_hook::h7d1c07b3a075203c
    3: 0x2dd18d - <unknown>!std::panicking::begin_panic_handler::{{closure}}::h3513767ae6c4d95c
    4: 0x2dd0ce - <unknown>!std::sys_common::backtrace::__rust_end_short_backtrace::h0bbf8ae75ca302c2
    5: 0x2dda18 - <unknown>!rust_begin_unwind
    6: 0x2e5141 - <unknown>!core::panicking::panic_fmt::h2e0bed4f7ae7c673
    7: 0x2e5f72 - <unknown>!core::result::unwrap_failed::h9a191c82353b6b6a
    8: 0x2809b7 - <unknown>!core::result::Result<T,E>::unwrap::h05f69224acb0968b
    9: 0x29424c - <unknown>!cgi::parse_request::h94a4e0bdc113cc08
   10: 0x81ed - <unknown>!cgi::handle::h67b331f056001c36
   11: 0xc9da - <unknown>!codewalk_wagi::main::had50194f91c9c7af
   12: 0x658c - <unknown>!core::ops::function::FnOnce::call_once::he17449d004b0050c
   13: 0xba4f - <unknown>!std::sys_common::backtrace::__rust_begin_short_backtrace::h70903421afc6fbec
   14: 0x6298 - <unknown>!std::rt::lang_start::{{closure}}::h3a7a3b5d2588242d
   15: 0x2de089 - <unknown>!std::rt::lang_start_internal::h1e63ab493c96e529
   16: 0x624a - <unknown>!std::rt::lang_start::hb40d56e937d3372a
   17: 0xcbe3 - <unknown>!__original_main
   18: 0x32bb - <unknown>!_start
   19: 0x2ea1fe - <unknown>!_start.command_export
note: run with `WASMTIME_BACKTRACE_DETAILS=1` environment variable to display more information
```

Eww... what is going on? In cases like this, take a look in the `logs/` directory. There,
you will find files that will have the captured error output from your `main.rs` code:

```
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: http::Error(InvalidUri(Empty))', /Users/technosophos/.cargo/registry/src/github.com-1ecc6299db9ec823/cgi-0.6.0/src/lib.rs:304:21
```

The error messages may be super helpful, or super terse... but at least now you've got some
line numbers to help you debug. (In this case, it was a problem with my route.)

If that doesn't help, here's another useful trick: You can try setting the environment
variable `RUST_LOG` before starting up WAGI:

```console
$ export RUST_LOG=warn,wagi=debug,wagi::runtime=debug
$ wagi -c ./modules.toml --log-dir ./logs
Aug 21 13:58:52.674  INFO wagi: Starting server addr=127.0.0.1:3000
Aug 21 13:58:52.676 DEBUG wagi: Env vars are set env_vars={}
```

With that enabled, we'll start seeing more information about what our Wagi server is doing.

## More content

We added another Markdown page this time around. And I beg of you, please make `content/about.md`
more interesting. Do it for the children. Or the dolphins. Or kittens.

## Some `modules.toml` magic

It's time to make some changes to `modules.toml`. this time, we're going to make our top
level route match subpages so that we can write one piece of code that can handle
`/` and `/about`, and whatever else we throw at it. Meander on over to `modules.toml` and
take a look at the new `route` entry.

Now, before coding, you can start up `wagi` and test out your route modification.
While the content will be the same for each page, you can try things like
`http://localhost:3000/about` or [http://localhost:3000/icky/icky/icky/ptang/zoop/boing](http://localhost:3000/icky/icky/icky/ptang/zoop/boing).

## Let's code

We've got some new content. We've modified our route. Maybe we've even tested it out.
Now it's time to modify some code. This time, we'll modify our `src/main.rs` to support
loading different documents depending on the route selected.

The new code focuses on loading a file based on the URL path. So once you're done reading
the code, you should be able to see `http://localhost:3000/` and then click to the `/about`
page.

## Take Me Forward! Take Me Back!

Head to chapter 6: `git checkout ch6-errors`
Return to the intro: `git checkout main`
