# Chapter 7: Handling Errors

Popular static site generators like Jekyll, Middleman, and Hugo have made popular the
concept of embedding "frontmatter" at the top of a Markdown document. This frontmatter
provides data about the document, and provides it in a format that the system can
programmatically use.

In this chapter, we add support for setting some simple frontmatter like `title`.
It will look like this:

```markdown
title = "Some Title"
---
This is the document
```

Everything above the first `---` is frontmatter. Go ahead and take a look at the files in
`content/` to see examples.

## More libraries!

We need to add a few new libraries to work with frontmatter. Our frontmatter is going to
follow the [TOML format](https://toml.io/en/). And we need to be able to deserialize
TOML documents. That's why we added a few libraries to our `Cargo.toml` (which, of course,
is also written in TOML).

## Code changes

Check out `src/main.rs` for the new code, which splits each Markdown file into two parts:
the frontmatter and the body.

- It parses the frontmatter into data that can be used by the program
- It passes the Markdown into the renderer
- Then all of that data is used to generate the final HTML page

Once you've looked at these, build and run the server and test it out. You can even
try out `http://localhost:3000/legacy` and see how it handles a Markdown document that
is missing its frontmatter.

Hey, know what would be cool to fix now? How about if we add real template support so
that we can define some prettier HTML?

## Take Me Forward! Take Me Back!

Head to chapter 8: `git checkout ch8-handlebars`
Return to the intro: `git checkout main`
