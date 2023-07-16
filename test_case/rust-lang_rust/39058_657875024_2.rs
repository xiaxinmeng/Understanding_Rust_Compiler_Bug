
error[E0369]: cannot add `()` to `()`
 --> src/main.rs:6:16
  |
6 |     let _ = () + ();
  |             -- ^ -- ()
  |             |
  |             ()

error[E0369]: cannot add `()` to `std::vec::Vec<{integer}>`
 --> src/main.rs:7:21
  |
7 |     let _ = vec![1] + ();
  |             ------- ^ -- ()
  |             |
  |             std::vec::Vec<{integer}>
