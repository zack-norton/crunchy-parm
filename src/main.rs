use std::time::{Duration, SystemTime};
use std::io;
use std::ops::Add;

struct Pomodoro {
    is_running: bool
}

impl Pomodoro {
    fn new() -> Self {
        return Pomodoro {
            is_running : false
        }
    }
}

fn main() {

    let mut pomo = Pomodoro::new();
    
    loop {
        println!("Enter command");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("failed to read line");

        match command.as_str() {
            "pomodoro" => {
                if !pomo.is_running {
                    pomo.is_running = true;
                    start_pomodoro_timer()
                }
            },
            "exit" => break,
            _ => {
                println!("Unknown command");
            }
        }
    }

    println!("Exiting...");
}

fn start_pomodoro_timer() {
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
