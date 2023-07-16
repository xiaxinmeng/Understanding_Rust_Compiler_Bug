rust
fn remove_by<F: FnMut(&T) -> bool>(&mut self, f: F)
