 rust
struct _X([u8]);

fn _f(x: &_X) { let ref _x = x.0; }

fn main() { }
