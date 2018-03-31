use std::rc::Rc;

fn main() {
    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    };

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }
    let l = Label { number: 3 };
    print(l);
    println!("My label num is {}", l.number);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "105");

    let second = v.swap_remove(1);
    assert_eq!(second, "102");

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    assert_eq!(third, "103");

    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("shirataki".to_string());
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
}
