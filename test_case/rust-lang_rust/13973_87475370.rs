 rust
fn main() {
    unsafe {
        static BLOCK_UNSAFE_SAFE_PTR: &'static isize = &*(0xdeadbeef as *const isize);
    }
}
