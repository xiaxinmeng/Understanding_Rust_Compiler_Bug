rust
static X: u8 = 0;
fn f<'a, 'b>(_x: &'b u8) -> &'a u8  { &X }
fn main() { let _ = f::<'static>; }
