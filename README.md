# Chapter 1: Configure

On our codewalk, we are going to build a simple microblog that runs in Wagi. While
you can use lots of different languages to write Wagi applications, we'll be using Rust.
While Rust is a notoriously difficult language to learn, we're making it easier by doing
two things:

1. The code will be simple and heavily commented
2. At any given moment in time, you can build the code straight from this git repo.

If you mess up at some point and break the code, you can always just use `git reset`
to get back to the starting version.

## What You Need

You will need the following things to do this tutorial:

1. You will definitely want Git. The good news is that you almost certainly have it already.
2. [Install Rust](https://www.rust-lang.org/tools/install) <-- See? We linked you straight to the install docs!
3. Configure the Rust compiler: `rustup target add wasm32-wasi`
4. [Install Wagi](https://github.com/deislabs/wagi/#quickstart) (Just get to the part where you can run `wagi --help`)

If you've gotten those for things done, _congratulations!_ We're already done with the first
chapter.

## Take Me Forward! Take Me Back!

Head to chapter 2: `git checkout ch2-hello-world`
Return to the intro: `git checkout main`
