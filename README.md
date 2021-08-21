# Chapter 2: Hello World

You might notice that things are a little different this time. We took the liberty of
running `cargo init` for you. So now we've got a `Cargo.toml` file (the main config
file for Rust projects).

## To the Source!

If you take a look inside of `src/`, you'll see that `cargo` created a `main.rs` file
for us. We've already started editing it.

Go ahead and take a look at `src/main.rs`. It's all documented, and the code should
be really easy to follow... after all, it's just a `main()` function with three `println`
commands.

## Compile time

Once you're comfortable with that code, go ahead and build it:

```console
$ cargo build --target wasm32-wasi
   Compiling codewalk-wagi v0.1.0 (/Users/technosophos/Code/Rust/codewalk-wagi)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
```

Above, we are using `cargo build` to build a binary. We want the binary format to be
`--target wasm32-wasi`. (If we left this off, it would build a native binary.)

That is all there is to compiling a Wagi app. If something went wrong, you might
want to go take another look at the last chapter: `git checkout ch1-configure`.

## Run it in Wagi

In the last chapter, you set up `wagi`. To use Wagi, we need to create a simple
configuration file that maps an HTTP route to a Wasm file.

Go check out `modules.toml`. It's right there in the top-level directory.
And like our `main.rs` file, it's commented.


Now we can run our Hello World app using `wagi`:

```console
wagi -c ./modules.toml --log-dir ./logs 
```

- The `-c ./modules.toml` points `wagi` to the `modules.toml` file we just examined.
- The `--log-dir ./logs` tells Wagi to store logs in the `logs/` directory. We could skip this, and then Wagi will put logs in a temp dir.

## Try it

By default, Wagi will listen on port `3000` on localhost. So you can either point your
web browser to `http://localhost:3000/`, or you can use a commandline program like
`curl`:

```console
$ curl http://localhost:3000/
Hello, world!
```

The nice thing about `curl` is that we can drop on the `-v` flag and see the raw HTTP
data.

```console
$curl -v http://localhost:3000/
*   Trying ::1...
* TCP_NODELAY set
* Connection failed
* connect to ::1 port 3000 failed: Connection refused
*   Trying 127.0.0.1...
* TCP_NODELAY set
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET / HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.64.1
> Accept: */*
>
< HTTP/1.1 200 OK
< content-type: text/plain
< content-length: 14
< date: Sat, 21 Aug 2021 13:03:11 GMT
<
Hello, world!
* Connection #0 to host localhost left intact
* Closing connection 0
```

While we don't need to do that now, it's a nice trick to have in your pocket when you
need to do some debugging.

But while we're at it... see that `content-type: text/plain` in there? That's from the
first line we printed in `main.rs`.

## Try some stuff. We'll wait.

The great thing about having all of this in a git repo is that you can edit the code
with reckless abandon. If anything goes wrong, just use commands like `git reset`,
`git checkout $FILE`, or `git stash` to get back to things as they were when you started.

Your process will go like this:

- Edit some code
- Re-run the `cargo build --target wasm32-wasi` command
- Restart `wagi -c ./modules.toml --log-dir ./logs` (Use `CTRL-C` to stop the server)
- Test it with your browser or `curl`

In the next section, we'll add a library to make writing these programs easier.

## Take Me Forward! Take Me Back!

Head to chapter 3: `git checkout ch3-hello-cgi`
Return to the intro: `git checkout main`
