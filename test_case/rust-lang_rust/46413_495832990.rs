
error[E0597]: `stmt` does not live long enough
  --> src/main.rs:19:25
   |
19 |     let mut rows = Rows(&stmt);
   |                         ^^^^^ borrowed value does not live long enough
20 |     rows.map(|row| row).next()
   |     ------------------- a temporary with access to the borrow is created here ...
...
26 | }
   | -
   | |
   | `stmt` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::iter::Map<Rows<'_>, [closure@src/main.rs:20:14: 20:23]>`
   |
   = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.
