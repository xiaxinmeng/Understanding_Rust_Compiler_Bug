
error[E0594]: cannot assign to `x`, as it is a captured variable in a `Fn` closure
 --> src/main.rs:4:9
  |
4 |         x += 1;
  |         ^^^^^^ cannot assign
  |
help: consider changing this to accept closures that implement `FnMut`
 --> src/main.rs:3:14
  |
3 |       Box::new(move || {
  |  ______________^
4 | |         x += 1;
5 | |         x
6 | |     })
  | |_____^
