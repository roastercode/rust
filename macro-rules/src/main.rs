/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations ───────────┐
// Macro Rules ─────────────────────┘

// This is a simple macro named `say_hello`.
macro_rules! hellboy {
    // `()` indicates that the macro takes no argument.
    () => (
        // The macro will expand into the contents of this block.
        println!("What the hell!");
    )
}

fn main() {
    // This call will expand into `println!("Hello");`
    hellboy!()
}

