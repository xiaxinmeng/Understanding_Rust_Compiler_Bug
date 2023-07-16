rust
    /// Returns the amount of time elapsed from another instant to this one,
    /// or from this one to another one, whichever is non-negative.
    pub fn distance(&self, another: Instant) -> Duration;
