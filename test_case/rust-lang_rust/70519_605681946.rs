
error: generic arguments must come before the first constraint
  --> file33.rs:11:34
   |
11 | struct Bar<'a, A, F: Foo<T = (), A, 'a, 4>> {
   |                          ------  ^  ^^  ^ generic argument
   |                          |       |  |
   |                          |       |  generic argument
   |                          |       generic argument
   |                          the constraint is provided here
   |
help: move the constraint after the generic arguments
   |
11 | struct Bar<'a, A, F: Foo<'a, A, 4, T = ()>> {
   |                         ^^^^^^^^^^^^^^^^^^

error[E0747]: lifetime provided when a constant was expected
  --> file33.rs:11:37
   |
11 | struct Bar<'a, A, F: Foo<T = (), A, 'a, 4>> {
   |                                     ^^
   |
   = note: constant arguments must be provided before lifetime arguments

error: aborting due to 2 previous errors
