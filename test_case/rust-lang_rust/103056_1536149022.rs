rs
// Panics (`checked_sub` returned `None`)
Instant::now() - Duration::from_secs(i64::MAX as u64 + 100)
