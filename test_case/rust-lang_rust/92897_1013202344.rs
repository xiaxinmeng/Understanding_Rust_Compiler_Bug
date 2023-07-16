rust
#[no_mangle]
pub fn __atomic_load_4(arg: *const usize, _ordering: usize) -> usize {
    unsafe { *arg }
}
