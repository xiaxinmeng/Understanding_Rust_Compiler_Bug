
fn main() { let mut x = Box::new(0); { let mut y = &mut x; y = y; let mut z = &y; z; y; } x; }
