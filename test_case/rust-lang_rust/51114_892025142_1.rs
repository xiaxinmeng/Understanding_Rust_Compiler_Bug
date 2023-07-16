text
error[E0507]: cannot move out of `x` in pattern guard
 --> src/main.rs:7:34
  |
7 |         Some(x) if let () = drop(x) => {}
  |                                  ^ move occurs because `x` has type `NotCopy`, which does not implement the `Copy` trait
  |
  = note: variables bound in patterns cannot be moved from until after the end of the pattern guard
