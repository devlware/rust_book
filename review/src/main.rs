
use std::f64::consts;

fn main() {

    the_strings();
    
    more_vectors();
    
    iterators();
    vectors();
    
    //let x = factorial(20);
    let x = 10;
    println!("valor de x e: {}", x);
    println!("Hello, world!");
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1, res2);

    let mut res = 3.3;
    modifies(&mut res);
    println!("res is {}", res);

    let _bigint: i64 = 0;
    
    let pi: f64 = 3.1416;
    let x = pi/2.0;
    let cosine = x.cos();
    println!("cosine is {}", cosine);
    
    // Arrays and Slices
    let arr = [10, 20, 30, 40];
    let first = arr[0];
    println!("first {}", first);
    
    let mut anotherarr: [f64; 10] = [0.0; 10];
    for idx in 0..4 {
        anotherarr[idx] += consts::PI;
        println!("[{}] {}", idx, anotherarr[idx]);
    }

    for i in 0..4 {
        println!("[{}] = {}", i, arr[i]);
    }
    println!("length {}", arr.len());
    let res = sum(&arr);
    println!("sum {}", res);
    
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
    
    let ints1 = [1, 2, 3, 4, 5];
    let slice1 = &ints1[0..2];
    let slice2 = &ints1[1..];  // open range!

    println!("ints {:?}", ints1);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);
    
    more_slices();
}

fn sdump(s: &str) {
    println!("str '{}'", s);
}
fn the_strings() {
    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string

    sdump(text);
    sdump(&s);
    
    let mut s = String::new();
    // initially empty!
    s += "Aa ";
    s.push('H');
    s.push_str("ello");
    s.push(' ');
    s += "World!"; // short for `push_str`
    // remove the last char
    s.pop();

    assert_eq!(s, "Aa Hello World");
    for b in s.as_bytes() {
        print!("{}, ", b);
    }
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);

    assert_eq!(res, "hello [10,20,30]");
    
    println!("\n-----------------------");
    
    let text = "static";
    let string = "dynamic".to_string();

    let text_s = &text[1..];
    let string_s = &string[2..4];

    println!("slices {:?} {:?}", text_s, string_s);
    
    println!("\n-----------------------");
    
    let multilingual = "Hi! ¡Hola! привет!";
    for ch in multilingual.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", multilingual.len());
    println!("count {}", multilingual.chars().count());

    let maybe = multilingual.find('р');
    if maybe.is_some() {
        let hi = &multilingual[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string();
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}


fn more_vectors() {
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);
    
    let mut v1 = vec![1, 10, 5, 39, 38, 1, 2, 11, 2, 40, 7];
    v1.sort();
    for item in v1.iter() {
        println!("valor do item: {}", item);
    }
    // remover dupĺicatas dentro do vetor
    v1.dedup();
    println!("Valor do vetor depois de remover duplicatas.");
    for item in v1.iter() {
        println!("valor do item: {}", item);
    }
    // assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}

fn iterators() {
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
    
    let arr = [101, 202, 303];
    // Quando vou iterar por uma array preciso chamar o iter() como abaixo.
    for i in arr.iter() {
        println!("{}", i);
    }
    
    // slices will be converted implicitly to iterators...
    let slice = &arr;
    // In fact, it is more efficient to iterate over an array or slice this way
    // than to use for i in 0..slice.len() {} because Rust does not have to 
    // obsessively check every index operation.
    for i in slice {
        println!("{}", i);
    }
    
    // nao esquecer que esta soma vai de 0 ate 9.
    let sum: i32  = (0..10).sum();
    println!("sum was {}", sum);

    let sum: i64 = [10, 20, 30].iter().sum();
    println!("sum was {}", sum);
    
    
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
    
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}

fn dump(arr: &[i32]) {
    println!("dump");
    println!("arr is {:?}", arr);
}
fn vectors() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
    
    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}


fn more_slices() {
    println!("\nMore slices with get().");
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
    
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());
    
    let _maybe_last = slice.get(5);
    // let _last = if maybe_last.is_some() {
    //     *maybe_last.unwrap()
    // } else {
    //     -1
    // };
    
    let _last = *slice.get(5).unwrap_or(&-1);
}

fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn modifies(x: &mut f64) {
    *x *= consts::PI;
}

fn by_ref(x: &i32) -> i32 {
    *x + 1
}

#[allow(dead_code)]
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
