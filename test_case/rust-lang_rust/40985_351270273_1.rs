
error[E0599]: no method named `powi` found for type `{float}` in the current scope
 --> src/main.rs:2:26
  |
2 |     println!("{:?}", 2.0.powi(2));
  |                          ^^^^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use core::num::Float;
  |
