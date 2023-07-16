rust
pub fn speedy_fast(x: &mut Option<[u64; 10]>) -> Option<[u64; 10]> {
    std_replace(x, None)
}
// copied from std::mem::replace, just to be clear what's happening (same behavior with std::mem::replace)
fn std_replace(dest: &mut Option<[u64; 10]>, mut src: Option<[u64; 10]>) -> Option<[u64; 10]> {
    std::mem::swap(dest, &mut src);
    src
}

// manually inlined, should be the same? but isn't
pub fn big_slow_write(x: &mut Option<[u64; 10]>) -> Option<[u64; 10]> {
    let mut blah = None;
    std::mem::swap(x, &mut blah);
    blah
}
