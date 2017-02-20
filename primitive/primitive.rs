/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

/// Rust experimentations
/// Primitive in rust

fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float  = 3.0;  // `f64`
    let default_integer = 7;   // `i32`.

    let mut mutable = 12; // Mutable `i32`.

    // Error! The type of a variable can't be changed
    mutable = true;
}

