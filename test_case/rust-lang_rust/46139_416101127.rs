
    pub fn ptr_eq(this: &Self, other: &Self) -> bool {
        this.ptr.as_ptr() as *const () == other.ptr.as_ptr() as *const ()
    }
