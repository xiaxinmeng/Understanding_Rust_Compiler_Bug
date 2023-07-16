rust
fn main() {
    unsafe {
        std::ptr::write_volatile(&mut (0, 0), (0, 0));
    }
}
