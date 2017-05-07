fn main() {

    const MAX_POINTS: u32 = 100_000;
    println!("Value for the constant is: {}", MAX_POINTS);
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Example showing how we can change the type by shadowing the variable.
    let spaces = "    ";
    let spaces = spaces.len();

    // This shows how this cannot be done with mut and will fail compiling.
    let mut spaces = "    ";
    spaces = spaces.len();

    // Number literals and examples:
    let decimal = 98_222;
    let hex = 0xDEADBEEF;
    let octal = 0o77;
    let binary = 0b1010_1010;
    let byte = b'A';

    let x = 2.0; // f64;

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    // Tuple, general way of grouping some number of other values with a
    // variety of types into one compound.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    // Example of "destructuring"
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // Array in Rust:
    // Arrays can't have its size changed.
    let a = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    anothe_function();
}

fn anothe_function() {
    println!("Another function!");
}
