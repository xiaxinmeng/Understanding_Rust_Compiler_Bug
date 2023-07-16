
error[E0277]: the trait bound `String: Copy` is not satisfied
 --> test.rs:4:9
  |
1 | fn foo<T: Copy>(t: T) {}
  |           ---- required by this bound in `foo`
...
4 |     foo("".to_string());
  |         ^^^^^^^^^^^^^^ the trait `Copy` is not implemented for `String`

error: aborting due to previous error
