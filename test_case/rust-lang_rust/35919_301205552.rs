
error[E0382]: use of moved value: `y`
 --> test.rs:8:20
  |
5 |         let x = y;
  |             - value moved here
...
8 |     println!("{}", y);
  |                    ^ value used here after move
  |
  = note: move occurs because `y` has type `&mut i32`, which does not implement the `Copy` trait

error: aborting due to previous error
