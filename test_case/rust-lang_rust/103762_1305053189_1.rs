
$ rustc +1.63.0 test.rs
error[E0106]: missing lifetime specifier
 --> test.rs:3:26
  |
3 | fn foo(_: &Type<'_>) -> &'_ str { loop {} }
  |           ---------      ^^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say which one of argument 1's 2 lifetimes it is borrowed from
help: consider introducing a named lifetime parameter
  |
3 | fn foo<'a>(_: &'a Type<'_>) -> &'a str { loop {} }
  |       ++++     ++               ~~

error: aborting due to previous error
