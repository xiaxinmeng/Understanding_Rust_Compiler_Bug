rust
extern "C" {
    fn foreign(ptr: *const i32);
}

pub fn forloop(start: *const i32, end: *const i32) {
    for ptr in start..end {
        unsafe { foreign(ptr) }
    }
}

pub fn for_each(start: *const i32, end: *const i32) {
    (start..end).for_each(|ptr| unsafe { foreign(ptr) });
}
