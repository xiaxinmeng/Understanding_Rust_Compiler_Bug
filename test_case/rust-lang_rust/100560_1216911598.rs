
error[E0277]: the trait bound `String: Bar` is not satisfied
 --> /home/gh-compiler-errors/test.rs:6:9
  |
6 |     foo(String::new(), String::new());
  |         ^^^^^^^^^^^^^ the trait `Bar` is not implemented for `String`
  |
note: required by a bound in `foo`
 --> /home/gh-compiler-errors/test.rs:3:11
  |
3 | fn foo<T: Bar>(t: T, s: String) {}
  |           ^^^ required by this bound in `foo`
