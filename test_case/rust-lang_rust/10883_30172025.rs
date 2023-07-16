
#[no_std];
#[feature(macro_rules)];
#[path = "rust-core/core/mod.rs"]
mod core;

#[no_mangle]
pub unsafe fn malloc(_: uint) -> *mut u8 {
  let x: u8 = 2;
  return x as *mut u8;
}

pub fn main() {}
