 rust
fn hint_none<R, F: FnOnce() -> R>(_: F) -> Option<T> {None}
...
let mut maybe_foo = hint_none(mk_foo);
