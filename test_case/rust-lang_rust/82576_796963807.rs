rust
impl ToString for i8 {
    #[inline]
    fn to_string(&self) -> String {
        let mut buf = String::with_capacity(4);
        if self.is_negative() {
            buf.push('-');
        }
        buf.push_str(&self.unsigned_abs().to_string());
        buf
    }
}
