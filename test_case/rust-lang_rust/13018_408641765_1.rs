rust
#![crate_type = "lib"]
#[no_mangle]
pub fn assert_not_max_usize(x: usize) -> usize {
    if x == usize::max_value() {
        None.unwrap();
    }
    x
}
