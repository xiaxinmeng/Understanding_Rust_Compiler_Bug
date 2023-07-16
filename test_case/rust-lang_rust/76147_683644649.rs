rust
pub fn union<'a>(mut v: &'a mut String) {
    unsafe {
        let x: *mut &'a mut String = &mut v;
        let y: &'a mut String = v;
        debug_assert_ne!(*x, y);
    }
}
