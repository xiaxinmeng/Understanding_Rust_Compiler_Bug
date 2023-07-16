bash


error[E0734]: stability attributes may not be used outside of the standard library
 --> src/lib.rs:4:5
  |
4 |     #[rustc_const_stable(feature = "foo", since = "1.3.37")]
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0734`.
error: could not compile `playground`

To learn more, run the command again with --verbose.
