use std::time::{Instant, Duration};
use colored::Colorize;

pub struct Log {
    now : Instant,
    elapsed : Duration
}

impl Log {
    pub fn new() -> Self {
        Self {
            now : Instant::now(),
            elapsed : Instant::now().elapsed()
        }
    }
    pub fn info(&self, msg : String) {
        println!("{} :: {}", "[ INFO ]".green(), msg.cyan());
    }
    pub fn error(&self, msg : String) {
        eprintln!("{}  :: {}", "[ ERROR ]".red().bold(), msg.cyan());
    }
    pub fn time_start(&mut self) {
        self.now = Instant::now();
    }
    pub fn benchmark(&mut self, msg : String) {
        self.elapsed = self.now.elapsed();
        let elapsed_str = format!("{:.2?}", self.elapsed);
        println!("{} :: {} {} {}","[ TIME ]".bright_purple(), msg.cyan(), "in".cyan(), elapsed_str.bright_purple().bold());
    }
}

