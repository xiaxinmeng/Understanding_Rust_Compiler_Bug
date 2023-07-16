
error[E0599]: no method named `nonexistent_method` found for fn item `fn(_) -> Option<_> {Option::<_>::Some}` in the current scope
 --> src/main.rs:2:10
  |
2 |     Some.nonexistent_method();
  |          ^^^^^^^^^^^^^^^^^^ method not found in `fn(_) -> Option<_> {Option::<_>::Some}`
  |
  = note: `Some` is a function, perhaps you wish to call it

For more information about this error, try `rustc --explain E0599`.
