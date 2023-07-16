
   Compiling playground v0.0.1 (/playground)
error[E0382]: use of moved value: `x`
 --> src/main.rs:9:14
  |
5 |         let x;
  |             - move occurs because `x` has type `S`, which does not implement the `Copy` trait
...
9 |         drop(x);
  |              ^ value moved here, in previous iteration of loop

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.

