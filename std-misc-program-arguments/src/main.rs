/*
Rust - Std Misc Program Arguments
Licence    : GNU GPL v3 or later
Author     : Aurélien DESBRIÈRES
Mail       : aurelien(at)hackers(dot)camp
Created on : June 2017

Write with Emacs-nox ───────────────┐
Rust ───────────────────────────────┘
*/

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    // $ ./args arg1 arg2
    println!("I go {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}

