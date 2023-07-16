rust
let element = CString::new(content).unwrap();
unsafe { hoedown_buffer_puts(ob, element.as_ptr()); }
