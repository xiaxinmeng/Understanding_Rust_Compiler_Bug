
   Compiling compiler-bug-test v0.1.0 (/home/taladar/temp/20211223/compiler-bug-test)
error[E0277]: expected a `FnMut<(char,)>` closure, found `String`
 --> src/main.rs:5:7
  |
5 |     s.replace(s1, &s2);
  |       ^^^^^^^ expected an `FnMut<(char,)>` closure, found `String`
  |
  = help: the trait `FnMut<(char,)>` is not implemented for `String`
  = note: required because of the requirements on the impl of `Pattern<'_>` for `String`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `compiler-bug-test` due to previous error
