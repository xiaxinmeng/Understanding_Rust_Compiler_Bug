rust
let (ptr, len, cap) = string.into_bytes().into_raw_parts();
let v = unsafe { Vec::from_raw_parts(ptr as *mut c_char, len, cap) };
