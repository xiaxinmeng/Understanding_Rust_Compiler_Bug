rust
let vec = Vec::with_capacity(999);
let does_not_grow = vec.as_vec_that_does_not_grow();

does_not_grow.push(1);
does_not_grow.extend(1..100);
does_not_grow.extend_from_slice(&[1,2,3,4]);
