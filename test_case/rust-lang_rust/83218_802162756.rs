rust
#![feature(drain_filter)]

let mut vec = vec![1, 2, 3, 4];
vec.drain_filter(|x| if *x > 3 {
    true
} else {
    *x += 1;
    false
});
assert_eq!(vec, [2, 3, 4]);
