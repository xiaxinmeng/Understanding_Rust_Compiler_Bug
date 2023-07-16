rust
impl Instant {
    pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration>;
    pub fn saturating_duration_since(&self, earlier: Instant) -> Duration;
}
