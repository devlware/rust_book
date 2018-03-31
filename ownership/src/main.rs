fn main() {
    let mut ss = String::from("hello");
    ss.push_str(", world!");

    println!("{}", ss);

    // using the clone method the memory on heap for s1 is copied
    // for s2. This operation can be expensive in some cases.
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // The code below is valid and x is not invalidated because for
    // known types/sizes the variable is stored entirely on the stack.
    // There is no difference between deep and shallow copying here.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // List of types that accept the Copy trait.

    // All the integer types, like u32.
    // The boolean type, bool, with values true and false.
    // All the floating point types, like f64.
    // Tuples, but only if they contain types that are also Copy. (i32, i32)
    // is Copy, but (i32, String) is not.

    let s = String::from("hello"); // s comes into scope.

    //
    //
    //  The ownership of a variable follows the same pattern every time:
    //  assigning a value to another variable moves it. When a variable
    //  that includes data on the heap goes out of scope, the value will
    //  be cleaned up by drop unless the data has been moved to be owned
    //  by another variable.
    //
    //

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here.
                        //FIXME, this is incorret println!("{}", s);
    let x = 5; // x comes into scope.

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward.
    println!("{}", x);

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String.

    (s, length)
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope.
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope.
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
