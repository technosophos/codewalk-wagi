# Chapter 4: Markdown

Well now! We've come a long way already. And in this chapter we will amp things up a bit.

## Adding some static files

You may have noticed that we added a new directory called `content`. Inside of that is
a file called `index.md`. Feel free to go take a look at it. Edit way, too. Or add
your own stuff in that directory. In this chapter, we're going to take a markdown file from
that directory, and format and serve it.

## Mapping the `content/` directory in `modules.toml`

After you've looked around in the `content/` directory for a while, we can head on over
to the `modules.toml` file. There's a new `volumes` directive in there that you should
look at. It's the glue that will make our next bit of code work.

## A few new dependencies

Our program is getting a little more complex. So in this round, we are going to add
two new libraries:

- `markdown` will render our Markdown files into HTML
- `anyhow` is an error handling library that we will use to keep our code simple (now
  that we are doing things that could actually produce errors).

Take a look at the `Cargo.toml` file to see these new dependencies.

## It's coding time!

Okay, so far we have done the following:

- Add some new markdown files in `content/`
- Mapped `content/` to a filesystem inside of our Wagi module
- Added `markdown` and `anyhow` libraries for our Rust code to use

Now we can hop into `src/main.rs` and tie this all together. We overhaul the code to
read Markdown files off of the filesystem, render them into HTML, and then send them
to the browser. It's a big step up from "Hello World"... but it turns out we can do
it with only a few more lines of code.

Go ahead and read through the revised `main.rs`. Then we will head on to the next
part of the codewalk, where we manage multiple pages of Markdown instead of just one.

## Take Me Forward! Take Me Back!

Head to chapter 4: `git checkout ch5-multi-page`
Return to the intro: `git checkout main`
