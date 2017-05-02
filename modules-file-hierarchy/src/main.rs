/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Modules File Hierarchy in Rust

// split.rs
// This declaration will look for a file named `my.rs` or `my/mod.rs` and will
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

// my/mod.rs
// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccesssible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}

// my/nested.rs
pub fn function() {
    println!("called `my::nested::function()`");
}

// my/inaccessible.rs
#[allow(dead_code)]
pub fn public_function() {
    println!("called `my::inacdessible::public_function()`");
}


