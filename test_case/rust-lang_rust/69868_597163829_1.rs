
error[E0597]: `lock` does not live long enough
  --> src/main.rs:20:25
   |
20 |     if let Some(item) = lock.read().unwrap().get(&5) {
   |                         ^^^^----------------
   |                         |
   |                         borrowed value does not live long enough
   |                         a temporary with access to the borrow is created here ...
...
25 | }
   | -
   | |
   | `lock` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `std::sync::RwLockReadGuard`
   |
   = note: The temporary is part of an expression at the end of a block. Consider adding semicolon after the expression so its temporaries are dropped sooner, before the local variables declared by the block are dropped.
