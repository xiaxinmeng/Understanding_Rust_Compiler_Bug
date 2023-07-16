
error[E0425]: cannot find function `len` in this scope
 --> /home/gh-compiler-errors/test2.rs:2:13
  |
2 |     let x = len([1i32]);
  |             ^^^ not found in this scope
  |
help: use the `.` operator to call method `len` on `&[i32]`
  |
2 -     let x = len([1i32]);
2 +     let x = [1i32].len();
  |
