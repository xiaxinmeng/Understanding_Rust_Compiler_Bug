text
error[E0277]: `[{integer}; 3]` does not implement `IntoIterator`
 --> r.rs:3:14
  |
3 |     for i in a {}
  |              ^ borrow the array with `&` or call `.iter()` on it to iterate over it
  |
  = help: the trait `IntoIterator` is required for the target of a for loop
  = note: borrowing the array will allow it to be coerced to a slice
