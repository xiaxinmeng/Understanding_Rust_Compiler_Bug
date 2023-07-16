
error[E0027]: pattern does not mention field `0`
 --> src/main.rs:7:11
  |
7 |    if let Bar{} = f {
  |           ^^^^^ missing field `0`
  |
help: include the missing field in the pattern
  |
7 |    if let Bar { 0 } = f {
  |               ^^^^^
help: if you don't care about this missing field, you can explicitely ignore it
  |
7 |    if let Bar { .. } = f {
  |               ^^^^^^
