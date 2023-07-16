rust
pub fn union(v: &mut Vec<String>, x: usize, y: usize) {
    unsafe {
        let x: *mut &mut String = &mut v.get_unchecked_mut(x);
        let y: *mut &mut String = &mut v.get_unchecked_mut(y);
        debug_assert_ne!(x, y);
    }
}
