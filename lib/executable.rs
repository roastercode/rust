/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Link the executable and the library in Rust

// executable.rs
// Link to `library`, import items uinder the `rary`module
extern crate rary;

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}

