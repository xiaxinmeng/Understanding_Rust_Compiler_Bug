rust
let mut s = String::from("hello");
drop(s); // deinitialized s
/* cannot read from s here */
s = String::from("world"); // reinitialized
