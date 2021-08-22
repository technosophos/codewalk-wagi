# Chapter 9: Serving Images

Well now... we have arrived at our last feature: Serving images. This can be a tricky
bit of code. First, there's some content type negotiation. Usually, we'd have to add
a whole lot of boilerplate content type code. And, of course there is some file I/O
we'd need to write.

Or... we could just use an off-the-shelf image server written specifically for Wagi.

And doing that will highlight one last feature of Wagi: We can run multiple modules in the
same Wagi instance. Even third party modules.

So in this chapter, we won't even have to touch the `main.rs`.

## Using multiple Wasm modules

Wagi was built with the expectation that developers would combine different WebAssembly
modules to build applications. One of the easiest ways to do that is to direct specific
routes to specific modules.

This is really interesting for a few reasons:

- We could write each endpoint handler in a different language
- We can compose our apps from other people's modules
- We can maintain different pieces of our app in different Git repositories
- It is a lot easier to keep modules small and simple when we can divide the work.
  (For example, our binary fileserver is only 198k.)
- There are additional security benefits to separating out modules, and we'll see an example of that here.

In our example here, we fetched the latest version of the [Wagi fileserver](https://github.com/deislabs/wagi-fileserver). The Wagi fileserver is written in [Grain](https://grain-lang.org/)
and supports a wide range of file types, including images.

The new `modules/` directory in this chapter has one module in it: `fileserver.gr.wasm`.
That's the fileserver. To add it to our app, we add another `[[module]]` section in
`modules.toml`. Now is a good time to go take a look.

Start up the Wagi server again, and you will 

## A little bit more security

The important thing you will see in the new `modules.toml` is that the new fileserver 
mounts a volume to `images/`. Each module has its own volumes. It has access _only_ to its
own volumes. So, for example, our fileserver does not have access to the `content/` or `templates/`
directories that we allowed our other module to use.

You can even test this theory. The fileserver module will serve up all kinds of files, not
just images. So if we could try to type in direct paths to files under `content/` or `templates/`.
For example, you could try `http://localhost:3000/images/templates/main.hbs`. Because the
fileserver module's root filesystem _only_ contains the contents of `images/`, the fileserver
will not be able to access anything else.

## Take Me Forward! Take Me Back!

Head to chapter 10: `git checkout ch10-wrapping-up`
Return to the intro: `git checkout main`
