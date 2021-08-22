# Chapter 10: Wrapping Up

Well, there you have it! You've built a complete application using Wagi.
From here, you can try building your own tools. And you may want to try creating
applications with other languages like Swift, C, or AssemblyScript.

But before you go, we wanted to offer a few parting bits of advice.

## Recompile your app with `--release`

All along, we have been compiling debug versions of our app. And if you take a look,
that simple little chunk of code is a whopping 28M.

Guess what? You can cut 25M off with one compile flag:

```console
$ cargo build --target wasm32-wasi --release
```

Then you will need to point your `modules.toml` to the new binary. (Check out the one for
this chapter. We already made the adjustment.) We promise: You'll notice a HUGE speedup.
Just be warned that the compile time tends to take a little long.

## Know your CGI variables

In our Rust code, we did something like this:

```rust
request.headers().get("X-CGI-Path-Info")
```

There are over a dozen special headers (and environment variables) in Wagi. These mostly
correspond to the CGI versions. The most complete list is [in the Wagi docs](https://github.com/deislabs/wagi/blob/main/docs/environment_variables.md).

## Take Me Forward! Take Me Back!

Head to chapter 10: `git checkout ch10-wrapping-up`
Return to the intro: `git checkout main`
