
error[E0309]: the parameter type `T` may not live long enough
  |
help: consider adding an explicit lifetime bound...
  |
25| impl<T: 'a> Transmit<T, Ref> for () {
  |       ++++
