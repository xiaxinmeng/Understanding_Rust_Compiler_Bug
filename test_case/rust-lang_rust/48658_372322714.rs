rust
fn transaction<F: FnMut($int_type) -> Option<$int_type>>(&self, f: F, success: Ordering, failure: Ordering) -> bool;
