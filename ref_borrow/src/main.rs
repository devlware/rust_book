// The Rules of References

// Let’s recap what we’ve discussed about references:

// 1) At any given time, you can have either but not both of:
// - One mutable reference.
// - Any number of immutable references.
// 2) References must always be valid.

fn main() {
    let s1 = String::from("hello");

    // Here we have a reference to an object as a parameter, this way the function
    // do not take the ownership of the value.
    let len = calculate_length(&s1); // A reference that refers to the value of s1.
                                     // This way when the function ends the value will not be dropped when the reference
                                     // goes out of scope.
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("Diego");
    change(&mut s);

    // Code shows that when trying to make a second reference of s to r2 this is
    // an error. You can only have one mutable reference to a particular piece of data
    // in a particular scope.
    let r1 = &mut s;
    //FIXME this fail           let r2 = &mut s;

    try_simultaneous_reference();

    // Attention below
    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM
    //
    let reference_to_nothing = no_dangle();
}

// This works without any problems. Ownership is moved out, and nothing is deallocated.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

fn try_simultaneous_reference() {
    let mut s = String::from("diego antunes");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// A data race is a particular type of race condition in which these three behaviors occur:
//
// Two or more pointers access the same data at the same time.
// At least one of the pointers is being used to write to the data.
// There’s no mechanism being used to synchronize access to the data.
