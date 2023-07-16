rust
let a: Option<i32> = ...;
let b: Option<i32> = ...;
let res = a.zip_with(b, Add::add);
// or
let res = a.zip(b).map(|(a, b)| a + b); // destruction needed 
