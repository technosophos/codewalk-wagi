# Chapter 8: Adding Handlebars Templates

The [Handlebars](https://handlebarsjs.com/) template language is a broadly used format
for declaring HTML templates. In this chapter, we add support for Handlebars. When
it comes to actually doing the rendering, we are going to use the Rust `handlebars`
library. Since this requires JSON support, we have added some libraries for that as well.

Take a quick glance at `Cargo.toml` to see those new libraries.

## Show me the templates!

We've added a new directory called `templates/`, and we've added a single template
already. Take a look. It's pretty basic. Feel free to try out some edits.

### Template development

If you are tinkering with templates, go ahead and start up the Wagi server. Your templates
will reload with each request. This is due to the way Wagi works. It does not automatically
cache every page load.

## Another volume in `modules.toml`

Our Rust code now needs access to the `templates/` directory. So we have added a new
volume mounting in the `modules.toml` file.

With this new mounting, the template files will be available in `templates/*` inside of the
Rust code.

## The big refactoring

Okay, now we have a large (in terms of line count) refactor in the code. Namely, we will
remove that big `format!()` function that we've been using since Chapter 3, and we'll
replace it the Handlebars template renderer.

At this point, it makes sense to take a brief look backward and be proud of what we have
built. Our little project now has the following features:

- It creates a basic website, similar to those coming from static site generators like Jekyll
- It can load Markdown files and render them into HTML
- If a Markdown file has a preamble, it can use that information as well
- With Handlebars support, our app can load and render custom templates
- We have at least rudimentary error handling
- ~~We can serve images~~ Looks like we have one more thing to do

## Take Me Forward! Take Me Back!

Head to chapter 9: `git checkout ch9-images`
Return to the intro: `git checkout main`
