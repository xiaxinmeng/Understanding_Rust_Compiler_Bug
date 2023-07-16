
error[E0637]: `'_` cannot be used here
 --> src/a.rs:5:6
  |
5 | impl<'_> A<'_> {
  |      ^^ `'_` is a reserved lifetime name

error[E0261]: use of undeclared lifetime name `'a`
 --> src/a.rs:6:27
  |
6 |     pub(crate) fn new(a: &'a String) -> Self {
  |                           ^^ undeclared lifetime
