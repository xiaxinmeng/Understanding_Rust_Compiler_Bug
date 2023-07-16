
error[E0382]: use of moved value: `v`
  --> test.rs:26:5
   |
25 |     in v {42i32};
   |     ------------ value moved here
26 |     in v {42};
   |     ^^^^^^^^^ value used here after move
   |
   = note: move occurs because `v` has type `MyVec<i32>`, which does not implement the `Copy` trait

error: aborting due to previous error(s)
