 rust
static X: u8 = 128;
static Y: *const i8 = &X as *const _ as *const _;

fn main() {
    assert_eq!(unsafe { *Y }, -128);
}
