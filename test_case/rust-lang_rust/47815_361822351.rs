rust
error[E0229]: associated type bindings are not allowed here
  --> src/main.rs:11:50
   |
11 | impl<F:FnOnce(T0)->R, T0, R> FnOnce(Option<T0>)->R for OptionLift<F> {
   |                                                  ^ associated type not allowed here
