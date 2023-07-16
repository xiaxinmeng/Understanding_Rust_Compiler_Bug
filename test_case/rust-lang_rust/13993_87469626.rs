 rust
#![feature(libc)]
extern crate libc;

fn main() {
  println!("{:?}", 1.0 as *const libc::FILE); // Can't cast float to foreign.
}
