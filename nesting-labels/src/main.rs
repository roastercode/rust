/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Nesting Labels - Nesting Labels in Rust

#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the outer loop");
            // This would break only inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }
        
        println!("this point will never be reached");
    }
    
    println!("Exited the outer loop");
}
