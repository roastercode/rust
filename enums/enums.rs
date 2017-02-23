/* Aurélien DESBRIÈRES
aurelien(at)hackers(dot)camp
License GNU GPL latest */

// Rust experimentations
// Enum - enum in Rust

// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// Create an `enum` to classify someone. Note how both names
// adn type information together specify the variant:
// `Engineer != Scientist` and `Height(i32) != Weight(i32)`. Each
// is different and independent.
enum Person {
    // An `enum` may either be `unit-like`,
    Engineer,
    Scientist,
    // like tuple structs,
    Height(i32),
    Weight(i32),
    // or like structures.
    Info { name: String, height: i32 }
}

// A function which takes a `Person` enum as an argument and
// return nothing.
fn inspect(p: Person) {
    // Usage of an `enum` must cover all cases (irrefutable)
    // so a `match` is used to branch over it.
    match p {
        Person::Engineer  => println!("Is an Engineer!"),
        Person::Scientist => println!("Is a Scientist!"),
        // Destructure `i` from inside the `enum`.
        Person::Height(i) => println!("Has a Height of {}.", i),
        Person::Weight(i) => println!("Has a weight of {}.", i),
        // Destructure `ifot` into `name` and `height`.
        Person::Info { name, height } => {
            println!("{} is {} tall!", name, height);
        },
    }
}

fn main() {
    let person    = Person::Height(18);
    let amira     = Person::Weight(10);
    // `to_owned()` creates an owned `String` from a string slice.
    let dave      = Person::Info { name: "Dava".to_owned(), height: 72 };
    let rebecca   = Person::Scientist;
    let rohan     = Person::Engineer;

    inspect(person);
    inspect(amira);
    inspect(dave);
    inspect(rebecca);
    inspect(rohan);
}
