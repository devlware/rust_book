fn is_whitespace(text: &str) -> bool {
    text.chars().all(|c| c.is_whitespace())
}

fn main() {
    println!("Hello, world!");

    println!("is whitespace: {}", is_whitespace(" diego "));
    println!("is whitespace: {}", is_whitespace("diegoantunes"));
}
