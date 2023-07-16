rust
let mut x = String::new();
let p = &mut x as *mut String;
let y = x;
p.write(String::new());
