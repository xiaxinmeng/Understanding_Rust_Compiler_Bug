
error[E0191]: the value of the associated types `Output` (from the trait `std::ops::Add`), `Output` (from the trait `std::ops::Sub`) must be specified
 --> src/lib.rs:1:12
  |
1 | type Foo = std::ops::Add + std::ops::Sub;
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |            |
  |            associated type `Output` must be specified
  |            associated type `Output` must be specified
