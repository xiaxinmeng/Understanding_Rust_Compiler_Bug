rust
error[E0277]: the trait bound `&T: std::cmp::PartialEq<T>>` is not satisfied
  --> $DIR/file.rs:14:5
   |
14 |     &t == t;
   |     ^^^^^^^ can't compare `&T` with `T`
