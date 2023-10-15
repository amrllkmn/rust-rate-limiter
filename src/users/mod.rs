use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub struct User {
    pub id: u32,
    bucket: Arc<Mutex<u32>>,
}

impl User {
    pub async fn new(id: u32) -> User {
        let user = User {
            id,
            bucket: Arc::new(Mutex::new(0)),
        };
        // Clone user for the task.
        let user_clone = user.clone();
        tokio::spawn(async move {
            user_clone.start().await;
        });
        user
    }

    pub async fn start(&self) {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let mut bucket = self.bucket.lock().await;
            if *bucket < 10 {
                *bucket += 1
            }
        }
    }

    pub async fn consume(&self) {
        let mut bucket = self.bucket.lock().await;
        *bucket -= 1;
    }

    pub async fn bucket_is_empty(&self) -> bool {
        let bucket = self.bucket.lock().await;
        *bucket < 1
    }
}
