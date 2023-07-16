rust
/// Extensions for `Duration`
pub trait DurationExt : Copy {
    /// Returns true if self is equivalent to zero
    fn is_zero(self) -> bool;

    /// Get a zero duration
    fn zero() -> Duration {
        Duration::new(0, 0)
    }
}

impl DurationExt for Duration {
    #[inline]
    fn is_zero(self) -> bool {
        self.as_secs() == 0 && self.subsec_nanos() == 0
    }
}
