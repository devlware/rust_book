use std::time::{Duration, Instant};
use std::thread::sleep;
extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn dom_something_heavy() {
    let cnt = 0;
    let between = Range::new(2u64, 7);
    let mut rng = rand::thread_rng();
    sleep(Duration::new(between.ind_sample(&mut rng), 0));
}

fn main() {

    let mut interval = Duration::new(10, 0);; //seconds
    let millis = Duration::from_millis(13);
    interval = interval.checked_sub(millis).unwrap();

    loop {
        let now = Instant::now();
        dom_something_heavy();
        println!("Hello, world!");
        let later = Instant::now();
        println!("Elapsed time is: {:?}", later.duration_since(now));

        let need_to_sleep = interval.checked_sub(later.duration_since(now));
        println!("Going to sleep: {:?}", need_to_sleep);
        let delay = match need_to_sleep {
            Some(v) => sleep(v),
            None => sleep(Duration::new(5,0)),
        };
    }
}
