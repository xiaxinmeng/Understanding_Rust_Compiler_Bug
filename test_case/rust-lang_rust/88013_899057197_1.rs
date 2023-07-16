
error: this trait bound is already specified in the where clause
 --> src/lib.rs:5:15
  |
5 | pub fn bar<T: Foo>() where T: Foo {}
  |               ^^^
  |
note: the lint level is defined here
 --> src/lib.rs:1:9
  |
1 | #![deny(clippy::pedantic)]
  |         ^^^^^^^^^^^^^^^^
  = note: `#[deny(clippy::trait_duplication_in_bounds)]` implied by `#[deny(clippy::pedantic)]`
  = help: consider removing this trait bound
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#trait_duplication_in_bounds
