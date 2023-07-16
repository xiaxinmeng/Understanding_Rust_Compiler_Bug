console
error[E0597]: `mutex` does not live long enough
  --> src/main.rs:38:23
   |
38 |     write!(MutexGuard(&mutex), "") /* no semicolon */
   |            -----------^^^^^^-
   |            |          |
   |            |          borrowed value does not live long enough
   |            a temporary with access to the borrow is created here ...
39 | }
   | -
   | |
   | `mutex` dropped here while still borrowed
   | ... and the borrow might be used here, when that temporary is dropped and runs the `Drop` code for type `needs_early_drop::MutexGuard`
