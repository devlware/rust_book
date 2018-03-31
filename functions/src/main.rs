fn main() {
    println!("Hello, world!");

    another_function(4);

    let x = 5;

    // Calling a function and a macro is considered an expression.

    // Below we have a block which evaluates to 4.
    // the code inside the block is evaluated and bounded to y in the let
    // statement.
    let y = {
        let x = 3;
        x + 1 // ATTENTION, here we do not have a ;
              // EXPRESSIONS do not include ending semicolons.
    };

    println!("The value of y is: {}", y);

    let f = five();
    println!("Value for f in Statement is:{}", f);

    let x2 = plus_one(5);
    println!("Value for x2 is:{}", x2);
}

fn five() -> i32 {
    5 // this is an EXPRESSION.
}

// Both
fn plus_one(x: i32) -> i32 {
    x + 1 // this is also an EXPRESSION
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("valur for x is:{}", x);
}
