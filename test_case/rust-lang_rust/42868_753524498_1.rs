rust
fn f<'a>() {}
fn main() { let _ = f::<'static>; }
