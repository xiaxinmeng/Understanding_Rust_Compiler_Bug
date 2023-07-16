 rust
pub fn with_c_strings<T: ToCStr, U>(x: &[T], f: |*const *const libc::c_char| -> U) -> U {
    let c_strings: Vec<CString> = x.iter().map(|s| s.to_c_str()).collect();
    let ptrs = c_strings.iter().map(|cs| cs.as_ptr())
                        .chain(Some(ptr::null::<libc::c_char>()).move_iter())
                        .collect::<Vec<*const libc::c_char>>();

    f(ptrs.as_ptr())
}
