rs
// Works fine.
Instant::now() - Duration::from_secs(i64::MAX as u64) - Duration::from_secs(100)
