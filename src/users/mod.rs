use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
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
            } else {
                break;
            }
        }
    }

    pub fn consume(&mut self) {
        self.bucket -= 1
    }

    pub fn bucket_is_empty(&self) -> bool {
        self.bucket < 1
    }
}
