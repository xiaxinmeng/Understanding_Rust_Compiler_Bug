rust
pub enum Void {}
pub fn foo(xs: [(Void, u32); 1]) -> u32 { xs[0].1 }
