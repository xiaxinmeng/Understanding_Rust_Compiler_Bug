Rust
use std::fmt;
fn main() {
  let x: *const _ = 0 as *const _; // we don't want to allow this if...
  let y: Option<*const fmt::Debug> = Some(x) as _;
  // ^ later, we have x = *const fmt::Debug
}

fn not_ok() {
  let x = 0 as *const i32 as *const _ as *mut _; //~ ERROR ?
}
