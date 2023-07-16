
error[E0599]: no method named `get` found for type parameter `T` in the current scope
 --> src/main.rs:6:7
  |
6 |     t.get(420);
  |     --^^^-----
  |     | |
  |     | this is an associated function, not a method
  |     help: disambiguate the associated function for the candidate: `Test::get(t, 420)`
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in the trait `Test`
 --> src/main.rs:2:5
  |
2 |     fn get(a: i32) -> i32;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  = help: items from traits can only be used if the type parameter is bounded by the trait
