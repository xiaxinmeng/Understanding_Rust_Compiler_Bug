
error[E0599]: no method named `context` found for type `std::result::Result<(), ()>` in the current scope
 --> src/main.rs:4:9
  |
4 |     run.context("foo");
  |         ^^^^^^^ method not found in `std::result::Result<(), ()>`
  |
  = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope, perhaps add a `use` for it:
  |
1 | use quick_error::ResultExt;
  |
