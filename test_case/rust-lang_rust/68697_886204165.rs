rust
error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
 --> src/lib.rs:6:9
  |
6 |     bar(&mut x);
  |         ^^^^^^
  |         |
  |         cannot borrow as mutable
  |         try removing `&mut` here
