extern crate arrayvec;

use arrayvec::ArrayVec;

fn main() {
    let mut output = ArrayVec::<[u8; 13]>::new();
    println!("sizeof: {}", output.len());
    let broker: u16 = 0xAAFF;
    let b1: u8 = ((broker >> 8) & 0xff) as u8;
    let b2: u8 = (broker & 0xff) as u8;
    output.push(b1);
    output.push(b2);
    println!("sizeof: {}", output.len());
}
