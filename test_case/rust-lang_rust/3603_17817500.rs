 rust
pub fn consume<T>(mut v: ~[T], f: &fn(uint, v: T)) {
    unsafe {
        do as_mut_buf(v) |p, ln| {
            for uint::range(0, ln) |i| {
                // NB: This unsafe operation counts on init writing 0s to the
                // holes we create in the vector. That ensures that, if the
                // iterator fails then we won't try to clean up the consumed
                // elements during unwinding
                let x = intrinsics::init();
                let p = ptr::mut_offset(p, i);
                f(i, util::replace_ptr(p, x));
            }
        }

        raw::set_len(&mut v, 0);
    }
}
