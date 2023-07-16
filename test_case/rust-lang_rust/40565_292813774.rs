rust
error[E0277]: the trait bound `{integer}: std::ops::Add<std::option::Option<{integer}>>` is not satisfied
  --> $DIR/binops.rs:12:5
   |
12 |     1 + Some(1);
   |     ^^^^^^^^^^^ no implementation for `{integer} + std::option::Option<{integer}>`

error[E0277]: the trait bound `usize: std::ops::Sub<std::option::Option<{integer}>>` is not satisfied
  --> $DIR/binops.rs:13:5
   |
13 |     1 as usize - Some(1);
   |     ^^^^^^^^^^^^^^^^^^^^ no implementation for `usize - std::option::Option<{integer}>`

error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<std::result::Result<{integer}, _>>` is not satisfied
  --> $DIR/binops.rs:14:5
   |
14 |     1 == Ok(1);
   |     ^^^^^^^^^^ can't compare `{integer}` with `std::result::Result<{integer}, _>`
