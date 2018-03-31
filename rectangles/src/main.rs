#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn main() {
    let length1 = 50;
    let width1 = 30;

    let rect1 = (30, 50);
    let rect2 = Rectangle {
        length: 13,
        width: 31,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(length1, width1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    // In the beginning of the file we add the annotation to derive the Debug
    // trait and print the Rectangle instance usgin the debug formatting.
    println!("rect2 is {:?}", rect2);

    // The format used below inside {} is a good one to output more
    // complex structures.
    println!("rect2 is also {:#?}", rect2);
}

// The parameters are passed using two separate variables.
fn area(length: u32, width: u32) -> u32 {
    length * width
}

// Parameters are passed using Tuples. Just one parameter is used, adding a
// little bit of structure.
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Function where we add more meanig to the parameter.
// The parameter passed is an immutable borrow of a struct Rectangle instance.
// This way main retains the its ownership and can continue using rect2, which
// is the reason we use the & in the funcion signature.
fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
