
error[E0191]: the value of the associated types `Output` (from trait `std::ops::Add`), `Output` (from trait `std::ops::Sub`) must be specified
 --> file12.rs:1:12
  |
1 | type Foo = std::ops::Add + std::ops::Sub;
  |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ associated types `Output`, `Output` must be specified
