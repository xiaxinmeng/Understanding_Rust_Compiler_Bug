
error: `State` is forbidden as the type of a const generic parameter
  --> src/main.rs:10:33
   |
10 |     pub struct Machine<const S: State> {
   |                                 ^^^^^
   |
   = note: the only supported types are integers, `bool` and `char`
   = help: more complex types are supported with `#![feature(const_generics)]`
