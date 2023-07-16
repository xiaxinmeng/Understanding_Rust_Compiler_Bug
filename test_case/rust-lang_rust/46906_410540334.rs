 rust
fn make_vec() -> Vec<i32> {
    let mut dest = Vec::with_capacity(1);
    unsafe {
        ptr::write(dest.as_mut_ptr().offset(0), 17);
        dest.set_len(1);
    }
    dest
}
