rust
impl Duration {
    pub const fn new(secs: u64, nanos: u32) -> Duration;
    pub const fn checked_add(self, rhs: Duration) -> Option<Duration>;
    pub const fn saturating_add(self, rhs: Duration) -> Duration;
    pub const fn checked_sub(self, rhs: Duration) -> Option<Duration>;
    pub const fn saturating_sub(self, rhs: Duration) -> Duration;
    pub const fn checked_mul(self, rhs: u32) -> Option<Duration>;
    pub const fn saturating_mul(self, rhs: u32) -> Duration;
    pub const fn checked_div(self, rhs: u32) -> Option<Duration>;
}
