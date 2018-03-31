#![feature(getpid)]
use std::process;
#[macro_use]
extern crate chan;
extern crate chan_signal;

use std::env;
use std::thread;
use std::time::Duration;

use chan_signal::Signal;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "usage: match_args <string>
    Check whether given string is the answer.
    match_args {{increase|decrease}} <integer>
    Increase or decrease given integer by one."
    );
}

fn main() {
    println!("My pid is {}", process::id());

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let args: Vec<_> = env::args().collect();
    match args.len() {
        // no arguments passed
        1 => {
            println!("My name is 'match_args'. Try passing some arguments!");
        }
        // one argument passed
        2 => match args[1].parse() {
            Ok(42) => println!("This is the answer!"),
            _ => println!("This is not the answer."),
        },
        // one command and one argument passed
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            // parse the command
            match &cmd[..] {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    eprintln!("error: invalid command");
                    help();
                }
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let key = "KEY";
    env::set_var(key, "VALUE");
    assert_eq!(env::var(key), Ok("VALUE".to_string()));

    let key = "HOME";
    // match env::var(key) {
    //    Ok(val) => println!("{}: {:?}", key, val),
    //    Err(e) => println!("couldn't interpret {}: {}", key, e),
    //}

    //for (key, value) in env::vars() {
    //   println!("{}: {}", key, value);
    //}
    // Signal gets a value when the OS sent a INT or TERM signal.
    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);
    // When our work is complete, send a sentinel value on `sdone`.
    let (sdone, rdone) = chan::sync(0);
    // Run work.
    thread::spawn(move || run(sdone));

    // Wait for a signal or for work to be done.
    chan_select! {
        signal.recv() -> signal => {
            println!("received signal: {:?}", signal)
        },
        rdone.recv() => {
            println!("Program completed normally.");
        }
    }
}

fn run(_sdone: chan::Sender<()>) {
    println!("Running work for 5 seconds.");
    println!("Can you send a signal quickly enough?");
    // Do some work.
    thread::sleep(Duration::from_secs(5));

    // _sdone gets dropped which closes the channel and causes `rdone`
    // to unblock.
}
