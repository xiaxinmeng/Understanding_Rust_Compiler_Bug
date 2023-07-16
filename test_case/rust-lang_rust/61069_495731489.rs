rust
let mut x = foo();
x = bar(); // DropAndReplace
drop(x);
yield;
baz(); // may access `x` via raw pointers, `x` must be in generator state
// no Drop at the end of scope, only StorageDead
