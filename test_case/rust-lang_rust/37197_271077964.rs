rust
    unsafe fn from_raw(ptr: *const T) -> Rc<T> {
        let fake_rc: Self = mem::transmute(ptr);
        let fake_rc_target = fake_rc.as_ref() as *const _ as *const u8;
        mem::forget(fake_rc);
        let raw_ptr = ptr as *const u8;
        let rc_ptr = raw_ptr.offset(raw_ptr as isize - fake_rc_target as isize);
        let mut result = mem::transmute(ptr);
        ptr::write(&mut result as *mut _ as *mut *const u8, rc_ptr);
        result
    }
