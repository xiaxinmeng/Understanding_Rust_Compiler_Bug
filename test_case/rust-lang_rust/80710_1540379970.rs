
error[[E0599]](https://doc.rust-lang.org/stable/error_codes/E0599.html): no method named `get` found for type parameter `T` in the current scope
 --> src/lib.rs:6:7
  |
5 | fn kek<T: Test>(t: T) {
  |        - method `get` not found for this type parameter
6 |     t.get(420)
  |       ^^^ this is an associated function, not a method
  |
  = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in the trait `Test`
 --> src/lib.rs:2:5
  |
2 |     fn get(a: i32) -> i32;
  |     ^^^^^^^^^^^^^^^^^^^^^^
  = help: items from traits can only be used if the type parameter is bounded by the trait
help: disambiguate the associated function for the candidate
  |
6 |     <T as Test>::get(t, 420)
  |
help: the following trait defines an item `get`, perhaps you need to restrict type parameter `T` with it:
  |
5 | fn kek<T: Test + SliceIndex>(t: T) {
  |                ++++++++++++

For more information about this error, try `rustc --explain E0599`.
error: could not compile `playground` due to previous error
