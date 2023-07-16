rust
pub fn union<'a>(v: &'a mut Vec<String>, x: usize, y: usize) {
    unsafe {
        let x: *mut &'a mut String = &mut v.get_unchecked_mut(x);
        let y: &'a mut String = v.get_unchecked_mut(y);
        debug_assert_ne!(*x, y);
    }
}
