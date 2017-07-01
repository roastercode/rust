/*
Rust - Std Misc Child Process Wait
Licence    : GNU GPL v3 or later
Author     : Aurélien DESBRIÈRES
Mail       : aurelien(at)hackers(dot)camp
Created on : June 2017

Write with Emacs-nox ───────────────┐
Rust ───────────────────────────────┘
*/

// Wait.rs

use std::process::Command;

fn main() {
    let mut child = Command::new("sleep").arg("5").spawn().unwrap();
    let _result = child.wait().unwrap();

    println!("reached end of main");
}
