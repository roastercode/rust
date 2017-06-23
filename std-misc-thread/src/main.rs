/*
Rust - Std Misc Thread
Licence    : GNU GPL v3 or later
Author     : Aurélien DESBRIÈRES
Mail       : aurelien(at)hackers(dot)camp
Created on : June 2017

Write with Emacs-nox ───────────────┐
Rust ───────────────────────────────┘
*/

use std::thread;

static NTHREADS: i32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        // Wait for the thread to finnish. Returns a result.
        let _ = child.join();
    }
}

