rust
mod std::sys::unix::time {
    use time::Duration;

    Instant {
        pub fn from_duration(duration: Duration) -> Instant;
        pub fn to_duration(self) -> Duration;
    }
}
