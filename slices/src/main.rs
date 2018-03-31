// Slices is item 4.3 from the TRPL book.
//
// Slice does not have ownership!
//
// Slice let you reference a contiguous sequence of elements in a collection rather
// than the whole collection.

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5.

    // The following line will crash on compiling.
    //s.clear();  // This empties the String, making it equal to "".

    let hello = &s[0..5];
    let world = &s[6..11];

    // With Rust’s .. range syntax, if you want to start at the first index
    // (zero), you can drop the value before the two periods. In other words,
    // these are equal:
    let outra_string = String::from("hello");

    let slice = &outra_string[0..2];
    let slice = &outra_string[..2];

    let name = String::from("Diego Wentz Antunes");
    let fword = get_first_word(&name);
    println!("first word is: {}", fword);

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = better_first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = better_first_word(&my_string_literal[..]);

    // since string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = better_first_word(my_string_literal);

    // Slices can also be used with other objects.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // This slice has the type &[i32]
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // Return from the beginning until the index of the ' '.
        }
    }

    // Return the full string, there is no space.
    &s[..]
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    // From the iter method it returns each element in a collection.
    // Enumerate wraps the result of iter and returns a tuple,
    // first element is the index and the second is a reference to the
    // element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
