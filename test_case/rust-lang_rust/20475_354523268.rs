rust
let mut buf = mem::uninitialized::<[c_char; 256]>();
let ptr = buf.as_ptr();
let len = buf.len();

/* ... */

// copy the fixed length stack buffer which may or may not
// have null terminator into an owned CString
CFixedStr::from_ptr(ptr, len).to_owned().into_c_string()
