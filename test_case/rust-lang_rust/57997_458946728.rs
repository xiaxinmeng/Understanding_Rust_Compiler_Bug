rust
fn main() {
    let mut x = 2;
    unsafe {
        core::intrinsics::write_bytes(&mut x, 0, 1);
    }
}
