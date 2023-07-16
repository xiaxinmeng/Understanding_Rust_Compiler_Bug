 rust
fn bar<'a, 'b, F: FnMut(&'a mut Nothing) -> &'b mut Nothing>(_: F) {}
