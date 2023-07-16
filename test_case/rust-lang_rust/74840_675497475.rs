rust
fn main() {
    let mut x: bool = true;
    let ptr = &mut x as *mut _ as *mut u8;
    unsafe { *ptr = 33; }
}
