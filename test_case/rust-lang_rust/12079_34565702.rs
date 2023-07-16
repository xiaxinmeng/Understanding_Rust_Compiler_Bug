 rust
fn volatile_load<T: Pod>(location: &T) -> T { ... }
fn volatile_store<T: Pod>(location: &mut T, t: T) { ... }
