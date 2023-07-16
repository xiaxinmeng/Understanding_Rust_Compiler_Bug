rust
impl Instant {
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
    pub fn checked_sub(&self, duration: Duration) -> Option<Instant> {
}
impl SystemTime {
    pub fn checked_add(&self, duration: Duration) -> Option<SystemTime> {
    pub fn checked_sub(&self, duration: Duration) -> Option<SystemTime> {
}
