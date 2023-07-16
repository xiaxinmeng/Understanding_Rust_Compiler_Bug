
error[E0599]: no method named `min` found for type `T` in the current scope
  --> file7.rs:18:19
   |
18 |                 e.min(n)
   |                   ^^^
   |
   = note: the method `min` exists but the following trait bounds were not satisfied:
           `&T : std::cmp::Ord`
           `&mut T : std::cmp::Ord`
           `&mut T : std::iter::Iterator`
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `min`, perhaps you need to restrict type argument `T` with one of them:
   |
11 | pub fn vec_min<T: Minimum/*: Minimum*/>(v: Vec<T>) -> SomethingOrNothing<T> {
   |                ^^^^^^^^^^
11 | pub fn vec_min<T: std::cmp::Ord/*: Minimum*/>(v: Vec<T>) -> SomethingOrNothing<T> {
   |                ^^^^^^^^^^^^^^^^
11 | pub fn vec_min<T: std::iter::Iterator/*: Minimum*/>(v: Vec<T>) -> SomethingOrNothing<T> {
   |                ^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error
