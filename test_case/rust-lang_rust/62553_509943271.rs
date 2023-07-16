rust
let mut local = 0;
let x = &mut local;
let raw = x as *mut _; // create raw pointer
some_function(x); // use x, re-asserting that x is unique
let _val = *raw; // use raw pointer -- UB because that would violate x's uniqueness
