rust
use async_std::future::timeout;
use std::time::Duration;

// Retry until not timeout, if TryFuture returns error, give up
while timeout(Duration::from_secs(1), ...some TryFuture...).await.transpose_err()?.is_err() {
}
