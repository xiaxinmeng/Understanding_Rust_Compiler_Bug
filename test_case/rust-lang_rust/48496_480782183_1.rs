
error[E0597]: `c` does not live long enough
  --> src/main.rs:13:27
   |
13 |             format!("{}", c.borrow())
   |                           ^---------
   |                           |
   |                           borrowed value does not live long enough
   |                           a temporary with access to the borrow is created here ...
14 |         })
   |         -
   |         |
   |         `c` dropped here while still borrowed
   |         ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::cell::Ref<'_, &str>`
   |
   = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.

error: aborting due to previous error
