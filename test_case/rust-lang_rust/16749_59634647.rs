 rust
let x = 0u;
...
let mut x_copy = x;
let f = move |&mut:| x_copy = 42;
