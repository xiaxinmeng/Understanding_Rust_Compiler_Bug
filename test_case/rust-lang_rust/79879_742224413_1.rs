
   Compiling playground v0.0.1 (/playground)
error[E0623]: lifetime mismatch
  --> src/lib.rs:12:34
   |
11 | fn errors(a: &mut A) {
   |              ------
   |              |
   |              these two types are declared with different lifetimes...
12 |     let mut b = child_with_value(a);
   |                                  ^ ...but data from `a` flows into `a` here

error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
