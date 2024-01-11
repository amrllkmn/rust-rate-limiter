use std::time::{Duration, Instant};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct User {
    pub addr: SocketAddr,
    bucket: Arc<Mutex<f32>>,
    last_refill: Instant,
}

const REFRESH_RATE_IN_SECONDS: f32 = 1.0;
const BUCKET_LIMIT: f32 = 10.0;

impl User {
    pub async fn new(addr: SocketAddr) -> User {
        User {
            addr,
            bucket: Arc::new(Mutex::new(0.0)),
            last_refill: Instant::now(),
        }
    }

    pub async fn allow_request(&mut self) -> bool {
        self.refill().await;
        let mut bucket = self.bucket.lock().await;
        if *bucket >= 1.0 {
            *bucket -= 1.0;
            return true;
        }
        false
    }

    pub async fn refill(&mut self) {
        let current_time = Instant::now();
        let time_elapsed = self.last_refill - current_time;
        if time_elapsed > Duration::new(1, 0) {
            let new_tokens = time_elapsed.as_secs_f32() * REFRESH_RATE_IN_SECONDS;
            let mut bucket = self.bucket.lock().await;
            if *bucket > BUCKET_LIMIT {
                *bucket += new_tokens;
                self.last_refill = Instant::now();
            }
        }
    }
}
