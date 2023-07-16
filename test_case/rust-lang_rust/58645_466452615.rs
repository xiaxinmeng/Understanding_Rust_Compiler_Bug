rust
fn main() {
    unsafe {
        struct S(u8, u8);
        std::ptr::write_volatile(&mut S(0, 0), S(0, 0));
    }
}
