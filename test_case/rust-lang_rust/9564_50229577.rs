 rust
fn with_c_strings<T: ToCStr, U>(x: &[T], f: |*const *const libc::c_char| -> U) -> U {
    let c_strings: Vec<CString> = x.iter().map(|s| s.to_c_str()).collect();
    let mut ptrs: Vec<*const libc::c_char> = c_strings.iter().map(|cs| cs.as_ptr()).collect();

    ptrs.push(ptr::null());

    f(ptrs.as_ptr())
}
