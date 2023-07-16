
error[E0277]: the trait bound `S: Bar` is not satisfied
  --> src/lib.rs:12:16
   |
12 |     data: [u8; <S as Bar>::BAR],
   |                ^^^^^^^^^^^^^^^ the trait `Bar` is not implemented for `S`
   |
   = help: consider adding a `where S: Bar` bound
note: required by `Bar::BAR`
  --> src/lib.rs:8:5
   |
8  |     const BAR: usize;
   |     ^^^^^^^^^^^^^^^^^
