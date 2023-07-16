rust
fn get_update<F>(&self, f: F) where T: Copy, F: FnMut(&mut T);
fn take_update<F>(&self, f: F) where T: Default, F: FnMut(&mut T);
