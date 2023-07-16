
error[[E0???]](https://doc.rust-lang.org/stable/error-index.html#E0???): bare trait invalid in this position
 --> src/lib.rs:1:34
  |
1 | fn make_iter<T>(vec: &Vec<T>) -> Iterator<Item = &T> {
  |                                  ^^^^^^^^^^^^^^^^^^^
  |
help: to indicate any concrete type implementing this trait, use the `impl` keyword
  |
1 - fn make_iter<T>(vec: &Vec<T>) -> Iterator<Item = &T> {
1 + fn make_iter<T>(vec: &Vec<T>) -> impl Iterator<Item = &T> {
  | 
help: to indicate a trait object with unknown type at compile time, use the `dyn` keyword
  |
1 - fn make_iter<T>(vec: &Vec<T>) -> Iterator<Item = &T> {
1 + fn make_iter<T>(vec: &Vec<T>) -> dyn Iterator<Item = &T> {
  | 

For more information about this error, try `rustc --explain E0???`.
