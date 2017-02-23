/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Testcase - Testcase in Rust

use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Dereference `self` and create a reference to `vec`
        // via destructuring.
        let List(ref vec) = *self;

        try!(write!(f, "["));

        // Iterate over `vec` in `v` while enumerating the iteration
        // count in `count`.
        for (count, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma
            // before calling `write!`. Use `try!` to return on errors.
            if count != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{}", v));
        }

        // Close the opened bracket and return a fmt::Result value
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![0: 1, 1: 2, 2: 3]);
    println!("{}", v);
}
