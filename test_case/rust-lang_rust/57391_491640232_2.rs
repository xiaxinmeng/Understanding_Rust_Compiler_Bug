rust
use std::time::{UNIX_EPOCH, Duration};

futures_timer::wait_for(30 * Duration::SECONDS).await;
for now in futures_timer::repeat(5 * Duration::MINUTES).await {
    println!("{} hours since the unix epoch", now - UNIX_EPOCH);
}
