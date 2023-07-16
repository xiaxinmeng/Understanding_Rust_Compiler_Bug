
fn main() { let mut x = 0; { let mut y = &mut x; y = y; y = y; y; } x; }
