
error[E0596]: cannot borrow immutable argument `self` as mutable
  --> src/main.rs:19:15
   |
19 |         (&mut self).bar(); //~ ERROR cannot borrow
   |               ^^^^
   |               |
   |               cannot reborrow mutably
   |               try removing `&mut` here
