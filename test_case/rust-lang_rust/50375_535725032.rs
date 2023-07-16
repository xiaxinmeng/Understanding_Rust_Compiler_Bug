
error[E0369]: binary operation `==` cannot be applied to type `Wrapper<T>`
 --> src/main.rs:7:7
  |
7 |     a == b
  |     - ^^ - Wrapper<T>
  |     |
  |     Wrapper<T>
  |
  = note: an implementation of `std::cmp::PartialEq` might be missing for `Wrapper<T>`
