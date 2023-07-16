
error[E0276]: impl has stricter requirements than trait
 --> test.rs:6:6
  |
2 |    fn bar<T: ?Sized>(&self, arg: &T);
  |    ---------------------------------- definition of `bar` from trait
...
6 |      fn bar<T>(&self, arg: &T) { }
  |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ impl has extra requirement `T: std::marker::Sized`

error: aborting due to previous error(s)
