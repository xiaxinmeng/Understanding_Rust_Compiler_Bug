
error[E0597]: `b` does not live long enough
 --> ../../issue-22449-ref-cell-if-let.rs:7:23
  |
7 |     if let Some(_x) = b.borrow().clone() {
  |                       ^---------
  |                       |
  |                       borrowed value does not live long enough
  |                       a temporary with access to the borrow is created here ...
8 |     }
9 | }
  | -
  | |
  | `b` dropped here while still borrowed
  | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, std::option::Option<i32>>`
  |
  = note: The temporary is part of an expression at the end of a block. Consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped.
