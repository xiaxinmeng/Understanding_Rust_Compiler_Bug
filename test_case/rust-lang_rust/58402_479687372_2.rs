rust
if let Some(time_left) = deadline.checked_duration_since(now) {
    sleeper.sleep(time_left);
}
