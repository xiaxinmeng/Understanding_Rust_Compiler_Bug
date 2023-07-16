plain
running 59 tests
.........ii.......F........................................
failures:

---- src/let_underscore.rs - let_underscore::LET_UNDERSCORE_LOCK (line 57) stdout ----
error: non-binding let on a synchronization lock
  --> src/let_underscore.rs:65:5
   |
10 |     let _ = data.lock().unwrap();
   |
   |
   = note: `#[deny(let_underscore_lock)]` on by default
help: consider binding to an unused variable
   |
10 |     let _unused = data.lock().unwrap();
help: consider explicitly droping with `std::mem::drop`
   |
   |
10 |     let _ = drop(...);

error: aborting due to previous error

Couldn't compile the test.
