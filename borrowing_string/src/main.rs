fn main() {
    let s1 = String::from("Rust"); // heap allocated string
    print_string(s1);
}

fn print_string(p1: String) {
    println!("{p1}");
}
