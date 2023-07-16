
    let p = self.as_mut_ptr();
    unsafe {
        for off in range(0, k / 2) {
            std::ptr::swap(p.offset(off), p.offset(k - 1 - off));
        }
    }
