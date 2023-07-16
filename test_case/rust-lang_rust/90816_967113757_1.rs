
error[E0477]: the type `Outer<'a, T>` does not fulfill the required lifetime
  --> src/lib.rs:16:5
   |
16 |     type AT<'b> = T::AT<'a>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: type must outlive the lifetime `'b` as defined here
  --> src/lib.rs:16:13
   |
16 |     type AT<'b> = T::AT<'a>;
   |             ^^

For more information about this error, try `rustc --explain E0477`.
