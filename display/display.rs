/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

/// Rust experimentations
/// Display in rust

// Import (via `use`) the `fmt` module to make it available
use std::fmt;

// Define a structure which `fmt::Display` will be implemented for. This is simpley
// a tuple struct containing an `i32` bound to the name `Structure`.
struct Strucutre(i32);

// In order to sue the `{}` marker, the trait `fmt::Display` must be implemented
// m
