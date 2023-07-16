
use std::iter;

let x: Vec<u8> = iter::repeat(0).take((1 << 31) + 1).collect();
x.iter().chain(x.iter())
