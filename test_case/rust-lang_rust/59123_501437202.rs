rust
let mut x = String::new();
let addr_x: *const String = reference_to_pointer(&x);
drop(x);
ptr::write(addr_x as *mut String, String::new());
