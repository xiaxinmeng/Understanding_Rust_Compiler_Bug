rust
use std::fmt::Debug;
fn foo(x: &u32, y: &u32) -> impl Debug {
  x
}
