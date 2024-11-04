use std::time::{Instant, Duration};

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
        println!("[INFO] :: {}", msg);
    }
    pub fn error(&self, msg : String) {
        eprintln!("[ERROR] :: {}", msg);
    }
    pub fn time_start(&mut self) {
        self.now = Instant::now();
    }
    pub fn time_end(&mut self) {
        self.elapsed = self.now.elapsed();
    }
    pub fn benchmark(&self, msg : String) {
        println!("[TIME] :: {} in {:.2?}", msg, self.elapsed);
    }
}

