 rust
#[start]
fn main(_: int, _: *const *const u8) -> int {
    unsafe {
        *(1234 as *mut u8) = 6;
    }
    0
}
