
error[E0271]: type mismatch resolving `<Result<&str, i32> as Try>::Output == &String`
 --> <source>:5:9
  |
5 |         std::ops::Try::from_output(&my_string)
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `str`, found struct `String`
  |
  = note: expected reference `&str`
             found reference `&String`
