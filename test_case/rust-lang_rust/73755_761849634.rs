rust
#[no_mangle]
pub extern fn greet(x: i32, y: i32) -> (i32, i32) {
  (x + y, x - y)
}
