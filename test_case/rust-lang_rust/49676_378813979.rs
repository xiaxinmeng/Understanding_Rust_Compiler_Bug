
error[E0530]: match bindings cannot shadow constants
 --> test.rs:9:7
  |
4 | pub const cmp: u8 = 1;
  | ---------------------- a constant `cmp` is defined here
...
9 |     B(u8)
  |       ^^^ cannot be named the same as a constant
