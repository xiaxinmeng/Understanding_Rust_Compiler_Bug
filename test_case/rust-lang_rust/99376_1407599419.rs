rust
let mut vec = Vec::with_capacity(1000);
vec.push(1);
let x: Box<[_]> = vec.into_iter().collect();
