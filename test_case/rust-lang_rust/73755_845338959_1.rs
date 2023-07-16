rust
...

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub fn load_model() -> (*mut c_char, usize)  {
        ...
        let pred: String = prediction[0].to_string();
        unsafe {
            return  (CString::from_vec_unchecked(pred.as_bytes().to_vec()).into_raw(), pred.len());
        }
}
