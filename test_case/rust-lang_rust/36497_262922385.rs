rust
pub fn rc_ptr_eq<T: ?Sized>(this: &Rc<T>, other: &Rc<T>) -> bool {
    let this_ptr: *const T = &*this;
    let other_ptr: *const T = &*other;
    this_ptr == other_ptr
}
