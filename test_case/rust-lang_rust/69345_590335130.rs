rust
fn main() {
    let x: Box<i32> = unsafe { Box::from_raw(16 as *mut i32) };
    std::mem::forget(x);
}
