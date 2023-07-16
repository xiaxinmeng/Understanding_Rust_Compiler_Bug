
error[E0277]: the trait bound `char: UpperHex` is not satisfied
 --> src/main.rs:3:19
  |
3 |     println!("{c} {c:X}");
  |                   ^^^^^ the trait `UpperHex` is not implemented for `char`
