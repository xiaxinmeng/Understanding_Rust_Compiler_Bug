 rust
let mut xs = [1, 2, 3];
let index = 1 >> 128; // this is `undef`
xs[index] = 100;
