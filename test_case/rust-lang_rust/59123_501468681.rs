rust
let mut x = String::new();
move_from(x);
// ...later...
x = bar();
let addr_x = &x as *const String;
move_from(x);
ptr::write(addr_x as *mut String, String::new());
