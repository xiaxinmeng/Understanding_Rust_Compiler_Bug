
fn banana<R, F: FnOnce() -> R>(f: F) -> R { f() }
