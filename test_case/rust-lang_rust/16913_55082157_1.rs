 rust
fn memcmp_eq<'a, T: PartialEq>(a: &'a [T], b: &'a [T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    unsafe {
        rlibc::memcmp(a.as_ptr() as *const _, b.as_ptr() as *const _, a.len()) == 0
    }
}
