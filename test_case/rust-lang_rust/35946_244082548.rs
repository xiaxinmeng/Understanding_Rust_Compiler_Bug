 bash
error[E0277]: the trait bound `(): std::ops::Carrier` is not satisfied
 --> src/main.rs:6:13
  |
6 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ trait `(): std::ops::Carrier` not satisfied
  |
  = note: required by `std::ops::Carrier::from_error`

error: aborting due to previous error
