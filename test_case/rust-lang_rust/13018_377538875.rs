rust
#![crate_type = "lib"]
#[no_mangle]
pub fn assert_not_max_usize(x: usize) -> usize {
    x.checked_add(1).unwrap().wrapping_sub(1)
}
