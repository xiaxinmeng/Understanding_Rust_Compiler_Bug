rust
fn add(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub fn add_ptr() -> fn(usize, usize) -> usize {
    add
}
