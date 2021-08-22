# Chapter 9: Serving Images

Well now... we have arrived at our last feature: Serving images. This can be a tricky
bit of code. First, there's some content type negotiation. Usually, we'd have to add
a whole lot of boilerplate content type code. And, of course there is some file I/O
we'd need to write.

Or... we could just use an off-the-shelf image server written specifically for Wagi.

And doing that will highlight one last feature of Wagi: We can run multiple modules in the
same Wagi instance. Even third party modules.



## Take Me Forward! Take Me Back!

Head to chapter 10: `git checkout ch10-wrapping-up`
Return to the intro: `git checkout main`
