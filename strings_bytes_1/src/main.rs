extern crate regex;

use regex::Regex;

fn main() {
    println!("Hello, world!");

    let speech = "\"Ouch!\" said the well.\n";

    println!("It was a bright, cold day in April, and \
    there were four of us—\
    more or less.");

    let default_win_install_path = r"C:\Program Files\Gorillas";
    let pattern = Regex::new(r"\d+(\.\d+)*");

    println!(r###"
        This raw string started with 'r###"'.
        Therefore it does not end until we reach a quote mark ('"')
        followed immediately by three pound signs ('###'):
    "###);

    let method = b"GET";
    assert_eq!(method, &[b'G', b'E', b'T']);
}
