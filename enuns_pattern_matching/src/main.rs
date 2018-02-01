// In this chapter we’ll look at enumerations, also referred to as enums.
// Enums allow you to define a type by enumerating its possible values.
// First, we’ll define and use an enum to show how an enum can encode meaning
// along with data. Next, we’ll explore a particularly useful enum, called
// Option, which expresses that a value can be either something or nothing.
// Then we’ll look at how pattern matching in the match expression makes it
// easy to run different code for different values of an enum. Finally, we’ll
// cover how the if let construct is another convenient and concise idiom
// available to you to handle enums in your code.

// Enums are a feature in many languages, but their capabilities differ in
// each language. Rust’s enums are most similar to algebraic data types in
// functional languages like F#, OCaml, and Haskell.

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


// Another option to define the enum without defining as extra struct is to use
// enum IpAddr {
//    V4(String),
//    V6(String),
//}
//
//let home = IpAddr::V4(String::from("127.0.0.1"));
//
//let loopback = IpAddr::V6(String::from("::1"));
//
// Rust also permitts that you define a enum where each part
// contains a define with different type as:
// enum IpAddr {
//      V4(u8, u8, u8, u8),
//      V6(String),
// }
//
// let home = IpAddr::V4(127, 0, 0, 1);
//
// let loopback = IpAddr::V6(String::from("::1"));
//
//
//

// The following code shows that you can put any kind of data inside an enum variant.
// strings, numeric types or structs for example. You can even include another enum.

//struct Ipv4Addr {
    // details elided
//}

//struct Ipv6Addr {
    // details elided
//}

//enum IpAddr {
//    V4(Ipv4Addr),
//    V6(Ipv6Addr),
//}

// Example of an enum with a variety of types.
//
// This enum has four variants with different types:

// Quit has no data associated with it at all.
// Move includes an anonymous struct inside it.
// Write includes a single String.
// ChangeColor includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let m = Message::Write(String::from("hello"));
    m.call();

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("os valores sao: {} - {} - {}", five, six, none);
}
