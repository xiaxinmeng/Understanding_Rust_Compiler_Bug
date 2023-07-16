 rust
extern {
    fn do_stuff(string: *mut RustAllocChar, free: fn(*mut RustAllocChar));
}

extern fn free(ptr: *mut RustAllocChar) {
    drop(CString::from_raw(ptr));
}

fn main() {
    let s = CString::new("Hello!").unwrap();
    unsafe {do_stuff(s.into_raw(), free)};
}
