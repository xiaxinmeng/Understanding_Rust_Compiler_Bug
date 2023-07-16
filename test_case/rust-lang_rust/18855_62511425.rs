
impl Duration {
    ...    
    pub fn as_is(&self) -> (i64, i32) {
        (self.secs, self.nanos)
    }
}
