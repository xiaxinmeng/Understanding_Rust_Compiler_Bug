rust
#[no_mangle]
pub extern "C" fn add_one(x: i32) -> i32 {
  let y = u128::max_value();
   let add = overflow_u128(x as u128, y);
  // let add = overflow_checked_u128(x as u128, y);
  add as i32
}

#[inline(never)]
fn overflow_u128(a: u128, b: u128) -> u128 {
  a + b
}

#[inline(never)]
fn overflow_checked_u128(a: u128, b: u128) -> u128 {
  a.checked_add(b).unwrap()
}
