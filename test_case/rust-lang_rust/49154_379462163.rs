rust
const __self_0: u8 = 0;

#[derive(Debug)]
pub enum Error {
    Type(
        &'static str,
    ),
}

fn main() {}

---- On stable

error[E0530]: match bindings cannot shadow constants
 --> src/main.rs:6:9
  |
1 | const __self_0: u8 = 0;
  | ----------------------- a constant `__self_0` is defined here
...
6 |         &'static str,
  |         ^^^^^^^^^^^^^ cannot be named the same as a constant
