/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations ──────────────────┐
// Error Handling Panice ──────────────────┘

fn give_princess(gift: &str) {
    // Princess hate snake, so we need to stop if she disapproves!
    if gift == "snake" { panic!("AAAAaaaa!!!"); }

    println!("I love {}s!!!!", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}

