rust
static TAB: [&mut [u8]; 0] = [];
pub unsafe fn test() {
    TAB[0].iter_mut();
}
pub fn main() {}
