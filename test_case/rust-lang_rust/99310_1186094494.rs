rust
let mut x = 0;
let f = || x += 1;
let y = &f as  &dyn Fn(); 
