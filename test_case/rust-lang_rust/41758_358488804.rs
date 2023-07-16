rust
fn resize_with(&mut self, new_len: usize, f: F)
where F: FnMut() -> T
