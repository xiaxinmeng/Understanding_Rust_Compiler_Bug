
error[E0599]: no method named `a` found for type `()` in the current scope
 --> src/main.rs:3:16
  |
3 | fn main() { ().a(); }
  |                ^
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
3 | use b::a::A;
  |
