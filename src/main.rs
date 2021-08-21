// This is a very basic Wagi program.
//
// It illustrates the absolute minimum amount of work you can do to build a
// wagi program.
fn main() {
    // Wagi reads the information you send to STDOUT (e.g. using `println`).
    // The first thing we need to do is tell Wagi what kind of information we
    // are sending it. And to do this, we use HTTP-style "headers".
    println!("content-type: text/plain");
    // We tell Wagi that we're done with the headers by printing an empty line.
    println!("");
    // Anything after that empty line gets sent straight back to the browser.
    println!("Hello, world!");
}
