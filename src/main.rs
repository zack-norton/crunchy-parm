use std::time::{Duration, SystemTime};
use std::ops::Add;

fn main() {
    println!("Pomodoro Timer");
    println!("Begin work");

    let stop_time = SystemTime::now()
        .add(
            Duration::from_secs(
                60 * 25 //1500 seconds = 25 minutes
            )
        );

    while SystemTime::now() < stop_time {}

    println!("Break time!");

    let stop_time = SystemTime::now()
        .add(
            Duration::from_secs(
                5 * 60 //300 seconds == 5 minutes
            )
        ); 

    while SystemTime::now() < stop_time {}

    println!("Break time over...");
}
