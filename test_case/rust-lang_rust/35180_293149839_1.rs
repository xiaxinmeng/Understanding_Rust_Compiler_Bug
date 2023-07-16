rust
error: main function not found

error[E0279]: the requirement `for<'a>  : 'a` is not satisfied (`expected bound lifetime parameter 'a, found concrete lifetime`)
 --> 1.rs:7:1
  |
7 | fn f(x: S<u32>) {}
  | ^^^^^^^^^^^^^^^^^^
  |
  = note: required because of the requirements on the impl of `for<'a> R<'a, 'static>` for `u32`
  = note: required by `S`

error: aborting due to previous error
