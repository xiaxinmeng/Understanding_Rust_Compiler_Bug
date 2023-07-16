rust
pub fn f(mut x: [Vec<u32>; 2]) {
    // three drops are emit: x[0], x[0], x[1], and an allocation for the new vector
    x[0] = vec![1, 2];
}
