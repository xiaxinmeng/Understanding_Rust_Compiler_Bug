rust
error[E0405]): cannot find trait `Trait` in this scope
 --> src/main.rs:4:10
  |
4 |     impl Trait for () {}
  |          ^^^^^ not found in this scope
  |
help: consider importing one of these items
  |
4 |     use crate::Trait;
  |
4 |     use traitobject::Trait;
  |
