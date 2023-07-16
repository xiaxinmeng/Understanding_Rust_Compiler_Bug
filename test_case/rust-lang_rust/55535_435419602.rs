
error[E0597]: `lock` does not live long enough
  --> src/lib.rs:16:23
   |
16 |     if let Some(_b) = lock.b() {}
   |                       ^^^^----
   |                       |
   |                       borrowed value does not live long enough
   |                       a temporary with access to the borrow is created here ...
17 | }
   | -
   | |
   | `lock` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and \
         runs the destructor for type `std::option::Option<impl std::iter::Iterator>`
   |
   = note: The temporary is part of an expression at the end of a block. \
     Consider adding semicolon after the expression so its temporaries are \
     dropped sooner, before the local variables declared by the block are dropped.
