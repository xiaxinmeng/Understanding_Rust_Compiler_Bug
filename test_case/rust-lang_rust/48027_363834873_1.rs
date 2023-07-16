
error[E0283]: type annotations required: cannot resolve `_: Bar`
 --> src/main.rs:3:32
  |
3 |     fn return_n(&self) -> [u8; Bar::X];
  |                                ^^^^^^
  |
  = note: required by `Bar::X`
