
fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}

fn main() {
    println!("Hello, world!");

    let lazy_caterer: [u32; 6] = [1,2,4,7,11,16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();
    assert_eq!(chaos, [1,2,3,4,5]);

    let mut v = vec![2, 3, 5, 7];
    assert_eq!(v.iter().fold(1, |a, b| a * b), 210);

    let mut vv = Vec::new();
    vv.push("step");
    vv.push("on");
    vv.push("no");
    vv.push("pets");

    let a: Vec<i32> = (0..5).collect();
    assert_eq!(a, [0,1,2,3,4]);

    let lang: Vec<String> = std::env::args().skip(1).collect();
    for l in lang {
        println!("{}: {}", l,
                 if l.len() % 2 == 0 {
                     "functional"
                 } else {
                     "imperative"
                 });
    }

    let z: Vec<f64> = vec![0.0,  0.707,  1.0,  0.707];
    let b: [f64; 4] =     [0.0, -0.707, -1.0, -0.707];

    let sz: &[f64] = &z;
    let sb: &[f64] = &b;

    print(&z[0..2]);    // print the first two elements of v
    print(&b[2..]);     // print elements of a starting with a[2]
    print(&sz[1..3]);   // print v[1] and v[2]

    println!("It was a bright, cold day in April, and \
    there were four of us—\
    more or less.");

}
