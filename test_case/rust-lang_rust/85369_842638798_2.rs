
error[E0277]: the trait bound `String: Copy` is not satisfied
 --> test.rs:4:9
  |
1 | fn foo<T: Copy>(t: T) {}
  |           ---- required by this bound in `foo`
...
4 |     foo("".to_string());
  |         ^^^^^^^^^^^^^^
  |         |
  |         expected an implementor of trait `Copy`
  |         help: consider borrowing here: `&"".to_string()`

error: aborting due to previous error
