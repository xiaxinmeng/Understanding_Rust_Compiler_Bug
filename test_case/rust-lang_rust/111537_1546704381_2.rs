
warning: unreachable statement
 --> poc.rs:3:5
  |
2 |     match true {};
  |     ------------- any code following this expression is unreachable
3 |     return;
  |     ^^^^^^^ unreachable statement
  |
  = note: `#[warn(unreachable_code)]` on by default

error[E0004]: non-exhaustive patterns: type `bool` is non-empty
 --> poc.rs:2:11
  |
2 |     match true {};
  |           ^^^^
  |
  = note: the matched value is of type `bool`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern as shown
  |
2 ~     match true {
3 +         _ => todo!(),
4 ~     };
  |
