
error[E0599]: no method named `context` found for type `std::result::Result<(), std::io::Error>` in the current scope
 --> src/main.rs:5:9
  |
5 |     run.context("foo");
  |         ^^^^^^^ method not found in `std::result::Result<(), std::io::Error>`
  |
  = help: items from traits can only be used if the trait is in scope
help: the following traits are implemented but not in scope, perhaps add a `use` for one of them:
  |
1 | use anyhow::Context;
  |
1 | use quick_error::ResultExt;
  |
