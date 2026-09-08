use std::time::Duration;
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

/// Holds the lock through the wait, so cancellation does not reserve future slots.
pub(crate) struct RateLimiter {
    next: Mutex<Instant>,
    interval: Duration,
}

impl RateLimiter {
    pub(crate) fn new(requests_per_second: u32) -> Self {
        Self {
            next: Mutex::new(Instant::now()),
            interval: Duration::from_secs_f64(1.0 / f64::from(requests_per_second)),
        }
    }

    pub(crate) async fn wait(&self) {
        let mut next = self.next.lock().await;
        sleep_until(*next).await;
        *next = Instant::now() + self.interval;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test(start_paused = true)]
    async fn spaces_concurrent_requests_without_a_burst() {
        let limiter = Arc::new(RateLimiter::new(2));
        let start = Instant::now();
        let mut tasks = Vec::new();
        for _ in 0..3 {
            let limiter = Arc::clone(&limiter);
            tasks.push(tokio::spawn(async move {
                limiter.wait().await;
                Instant::now()
            }));
        }
        let mut times = Vec::new();
        for task in tasks {
            times.push(task.await.unwrap());
        }
        times.sort();
        assert_eq!(
            times,
            vec![
                start,
                start + Duration::from_millis(500),
                start + Duration::from_secs(1)
            ]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_cancelled_wait_does_not_consume_an_extra_slot() {
        let limiter = Arc::new(RateLimiter::new(1));
        limiter.wait().await;
        let start = Instant::now();
        let copy = Arc::clone(&limiter);
        let task = tokio::spawn(async move { copy.wait().await });
        tokio::task::yield_now().await;
        task.abort();
        let _ = task.await;
        limiter.wait().await;
        assert_eq!(Instant::now() - start, Duration::from_secs(1));
    }
}
