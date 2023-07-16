
error[E0277]: the trait bound `[{integer}; 3]: std::iter::Iterator` is not satisfied
 --> src/main.rs:2:5
  |
2 |     for x in [1,2,3] {}
  |     ^^^^^^^^^^^^^^^^^^^ `[{integer}; 3]` is not an iterator; maybe try calling `.iter()` or a similar method
  |
  = help: the trait `std::iter::Iterator` is not implemented for `[{integer}; 3]`
  = note: required by `std::iter::IntoIterator::into_iter`
