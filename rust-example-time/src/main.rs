extern crate time;
extern crate chrono;

use std::thread;
use std::time::Duration;

fn main() {
    println!("Starting up at {}", chrono::UTC::now());

    let start = chrono::UTC::now();

    let sleep_time = Duration::new(0, 100000000);
    println!("Going to sleep...");
    thread::sleep(sleep_time);

    println!("I have woken up!");

    let end =  chrono::UTC::now();
    let duration: time::Duration = end - start;
    println!("Elapsed: {} milliseconds", duration.num_milliseconds() );

}
