text
error[E0277]: the trait bound `I: std::iter::Iterator` is not satisfied
 --> <anon>:6:5
  |
6 |     fn new<It: IntoIterator<IntoIter=I>>(iterable: It) -> Self {
  |     ^ trait `I: std::iter::Iterator` not satisfied
  |
  = help: consider adding a `where I: std::iter::Iterator` bound
  = note: required by `std::iter::IntoIterator`
