use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct User {
    id: u32,
    bucket: u32,
}

impl User {
    pub fn new(id: u32) -> User {
        User { id, bucket: 0 }
    }

    pub fn start(&mut self) {
        loop {
            if self.bucket < 10 {
                // Sleep for one second
                thread::sleep(Duration::from_secs(1));
                self.bucket += 1
            }
        }
    }

    pub fn consume(&mut self) {
        self.bucket -= 1
    }
}
