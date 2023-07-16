 rust
use std::libc;

fn callback(_: ~fn()) {}

unsafe fn foo() {
  do callback {
    libc::exit(1);
  }
}

fn main() {
}
