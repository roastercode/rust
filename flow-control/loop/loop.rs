/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Flow Control - loop in Rust

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }
        
        if count == 6 {
            println!("six");
            continue;
        }

        println!("{}", count);

        if count == 10 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}

