rust
error[E0277]: the trait bound `{integer}: std::ops::Add<std::option::Option<{integer}>>` is not satisfied
 --> <anon>:3:5
  |
3 |     1 + Some(1);
  |     ^^^^^^^^^^^ the trait `std::ops::Add<std::option::Option<{integer}>>` is not implemented for `{integer}`
  |
  = note: no implementation for `{integer} + std::option::Option<{integer}>`

error[E0277]: the trait bound `usize: std::ops::Sub<std::option::Option<{integer}>>` is not satisfied
 --> <anon>:4:5
  |
4 |     1 as usize - Some(1);
  |     ^^^^^^^^^^^^^^^^^^^^ the trait `std::ops::Sub<std::option::Option<{integer}>>` is not implemented for `usize`
  |
  = note: no implementation for `usize + std::option::Option<{integer}>`

error[E0277]: the trait bound `{integer}: std::cmp::PartialEq<std::result::Result<{integer}, _>>` is not satisfied
 --> <anon>:5:5
  |
5 |     1 == Ok(1);
  |     ^^^^^^^^^^ the trait `std::cmp::PartialEq<std::result::Result<{integer}, _>>` is not implemented for `{integer}`
  |
  = note: can't compare `{integer}` with `std::result::Result<{integer}, _>`
