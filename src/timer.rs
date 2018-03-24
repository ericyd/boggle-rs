// extract some of the logic for getting the user's playing time,
// getting remaining time, and checking if time is up

use std::time::SystemTime;
use std::io;

pub struct Timer {
    start_time: SystemTime,
    max_time_secs: i64,
    pub max_time_minutes: f64,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: SystemTime::now(),
            max_time_secs: 0,
            max_time_minutes: 0.0,
        }
    }

    pub fn get_remaining_time(&self) -> i64 {
        match self.start_time.elapsed() {
            Ok(elapsed) => self.max_time_secs - elapsed.as_secs() as i64,
            // If clock goes backwards, alert user
            Err(e) => {
                println!("Error: {:?}", e);
                -1
            }
        }
    }

    pub fn is_time_up(&self) -> bool {
        self.get_remaining_time() < 0
    }

    pub fn start(&mut self) {
        self.start_time = SystemTime::now()
    }

    pub fn get_user_play_time(&mut self) {
        // get total game length
        let mut max_time_minutes = String::new();
        loop {
            io::stdin().read_line(&mut max_time_minutes).expect(
                "Failed to read line",
            );
            match max_time_minutes.trim().parse::<f64>() {
                Ok(n) => {
                    self.max_time_minutes = n;
                    self.max_time_secs = (n * 60.0) as i64;
                    break;
                }
                Err(_e) => {
                    println!("Enter a whole number!");
                    // great, now we have to clear play_time_minutes and start_time again...
                    max_time_minutes = String::new();
                }
            }
        }
    }
}
