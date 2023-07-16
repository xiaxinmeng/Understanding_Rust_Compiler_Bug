
error[E0276]: impl has stricter requirements than trait
  --> src/lib.rs:11:15
   |
4  |     type Assoc<'a>;
   |     -------------- definition of `Assoc` from trait
...
11 |         Self: 'a;
   |               ^^ impl has extra requirement `'b: 'a`
