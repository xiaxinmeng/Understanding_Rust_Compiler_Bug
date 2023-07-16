rust
unsafe fn from_raw(ptr: *const T) -> Rc<T> {
    // Create a temporary fake Rc object from the given pointer and
    // calculate the address of the inner T.
    let fake_rc: Self = mem::transmute(ptr);
    let fake_rc_target = fake_rc.as_ref() as *const _;
    mem::forget(fake_rc);

    // Calculate the offset of T in RcBox<T>
    let rc_offset = (fake_rc_target as *const u8) as isize - (ptr as *const u8) as isize;

    // Get the address of the RcBox<T> by subtracting the offset from the
    // pointer we were originally given.
    let rc_ptr = (ptr as *const u8).offset(-rc_offset);

    // If T is an unsized type, then *const T is a fat pointer. To handle
    // this case properly we need to preserve the second word of the fat
    // pointer but overwrite the first one with our adjusted pointer.
    let mut result = mem::transmute(ptr);
    ptr::write(&mut result as *mut _ as *mut *const u8, rc_ptr);
    result
}
