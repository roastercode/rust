// Aurélien DESBRIÈRES
// aurelien(at)hackers(dot)camp
// License GNU GPL latest

/// Rust experimentations
/// Comments
/// comment.rs

fn main() {
    // This is an example of a line comment
    // Notice how there a re two slashes ate the beginning of the line
    // And that nothing written inside these will be read by the compiler

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashe, and run it again.

    /*
     * Another type of comment
     */

     /*
     Just my comment style
     */

    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}
