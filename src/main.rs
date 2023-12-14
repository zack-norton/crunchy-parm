use std::time::{Duration, SystemTime};
use std::ops::Add;

fn main() {
    println!("Pomodoro Timer");
    println!("Begin work");

    let stop_time = SystemTime::now().add(Duration::from_secs(5));

    while SystemTime::now() < stop_time {}

    println!("Break time!");

}
