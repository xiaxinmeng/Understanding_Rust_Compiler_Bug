rust
let x = String::from("hello");
let x_ptr = &mut x as *mut String;
std::mem::drop(x);
unsafe { *x_ptr = String::from("goodbye"); }
