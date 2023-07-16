rust
pub fn resize_with<F>(&mut self, new_len: usize, f: F)
where
    F: FnMut() -> T,
{
    let len = self.len();
    if new_len > len {
        self.extend_with(new_len - len, ExtendFunc(f));
    } else {
        self.truncate(new_len);
    }
}
