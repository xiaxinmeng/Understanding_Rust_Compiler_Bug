rust
error[E0121]: the type placeholder `_` is not allowed within types on item signatures
 --> <anon>:1:11
  |
1 | fn f() -> _ { || 0 }
  |           ^ not allowed in type signatures
  |
  = help: consider using an `Fn`, `FnMut`, or `FnOnce` trait bound
  = note: For more information on using `Fn` traits with function outputs, see https://doc.rust-lang.org/rust-by-example/fn/closures/output_parameters.html

error: aborting due to previous error

For more information about this error, try `rustc --explain E0121`.
