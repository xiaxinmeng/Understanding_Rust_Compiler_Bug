rust
static FOO: [u8; 1] = [0];
static TAB: [&mut [u8]; 1] = [&mut FOO];
pub unsafe fn test() {
    TAB[0].iter_mut();
}
pub fn main() {}
