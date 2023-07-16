
error[E0382]: use of moved value: `f`
 --> src/main.rs:5:27
  |
5 |             repeat(n - 1, f);
  |                           ^ value moved here, in previous iteration of loop
  |
  = note: move occurs because `f` has type `F`, which does not implement the `Copy` trait

error: aborting due to previous error
