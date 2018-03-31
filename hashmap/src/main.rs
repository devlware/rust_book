use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Create a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    // type inference lets us omit an explicit type signature (which
    // would be `HashMap<&str, &str>` in this example).
    let mut book_reviews = HashMap::new();

    // review some books.
    book_reviews.insert("Adventures of Huckleberry Finn", "My favorite book.");
    book_reviews.insert("Grimms' Fairy Tales", "Masterpiece.");
    book_reviews.insert("Pride and Prejudice", "Very enjoyable.");
    book_reviews.insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.");

    // check for a specific one.
    if !book_reviews.contains_key("Les Misérables") {
        println!(
            "We've got {} reviews, but Les Misérables ain't one.",
            book_reviews.len()
        );
    }

    // oops, this review has a lot of spelling mistakes, let's delete it.
    book_reviews.remove("The Adventures of Sherlock Holmes");

    // look up the values associated with some keys.
    let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
    for book in &to_find {
        match book_reviews.get(book) {
            Some(review) => println!("{}: {}", book, review),
            None => println!("{} is unreviewed.", book),
        }
    }

    // iterate over everything.
    for (book, review) in &book_reviews {
        println!("{}: \"{}\"", book, review);
    }

    println!("Hello, world!");

    // Use a HashMap to store the vikings' health points.
    let mut vikings = HashMap::new();

    vikings.insert(Viking::new("Einar", "Norway"), 25);
    vikings.insert(Viking::new("Olaf", "Denmark"), 24);
    vikings.insert(Viking::new("Harald", "Iceland"), 12);

    // Use derived implementation to print the status of the vikings.
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}
