
   Compiling playground v0.0.1 (/playground)
error[E0271]: type mismatch resolving `<[closure@src/main.rs:12:28: 12:36 x:_] as std::ops::FnOnce<()>>::Output == ()`
  --> src/main.rs:13:9
   |
2  |     fn do_twice<F>(mut func: F)
   |        -------- required by a bound in this
3  |         where F: FnMut()
   |                  ------- required by this bound in `main::do_twice`
...
13 |         do_twice(add_two_to_x);
   |         ^^^^^^^^ expected `()`, found `usize`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0271`.
error: could not compile `playground`.

To learn more, run the command again with --verbose.
