fn main() {
    // Slices are references to a contiguous sequence of
    // elements in a collection.

    let tweet = String::from(
        "This is my tweet and it's very very long"
    );

    let tweet2 = String::from(
        "This is my tweet and it's very long"
    );

    let trimmed_tweet: &str = trimed_tweet(&tweet); // string slice

    //let tweet2 = "This is my tweet and it's very long";
    let trimmed_tweet2: &str = trimed_tweet2(&tweet2);

    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let a_slice = &a[..3];

    println!("{}", trimmed_tweet);
    println!("{}", trimmed_tweet2);
    println!("{:#?}", a_slice);
}

fn trimed_tweet(tweet: &String) -> &str {
    &tweet[..20]
}

fn trimed_tweet2(tweet2: &String) -> &str {
    &tweet2[20..]
}
