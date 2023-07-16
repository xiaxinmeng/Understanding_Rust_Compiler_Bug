rust
// revisions:rpass1 rpass2
// See issue #58813

#![allow(warnings)]
pub trait T1: T2 { }
#[cfg(rpass1)]
pub trait T2 { }
#[cfg(rpass2)]
pub trait T2: T1 { }

fn main() { }
