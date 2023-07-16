rust
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::<()>::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}
