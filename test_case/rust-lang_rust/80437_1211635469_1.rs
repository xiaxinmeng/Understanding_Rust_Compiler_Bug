rust
#![feature(box_into_inner)]

let a = Box::new(5);
let b = Box::into_inner(a); // move occurs because `a` has type `Box<i32>`, which does not implement the `Copy` trait
let c = *a; // error, use of moved value: `a`
