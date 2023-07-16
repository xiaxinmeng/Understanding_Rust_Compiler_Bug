rust
pub unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut n = 0;
    while *s != 0 {      // <-- loop & deref
        n += 1;
        s = s.offset(1); // <-- raw pointer offset
    }
    return n
}
