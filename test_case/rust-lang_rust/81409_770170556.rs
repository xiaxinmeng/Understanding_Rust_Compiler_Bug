rust
fn count(self) -> usize {
    self.iter.filter(|&&byte| !utf8_is_cont_byte(byte)).take(usize::MAX).count()
}
