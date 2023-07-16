
error[E0277]: the trait bound `{integer}: std::ops::Carrier` is not satisfied
 --> src/main.rs:4:5
  |
4 |     42?;
  |     ^^^ trait `{integer}: std::ops::Carrier` not satisfied
  |
  = help: the following implementations were found:
  = help:   <std::ops::_DummyErrorType as std::ops::Carrier>
  = help:   <std::result::Result<U, V> as std::ops::Carrier>
  = note: required by `std::ops::Carrier::translate`

error[E0277]: the trait bound `(): std::ops::Carrier` is not satisfied
 --> src/main.rs:4:5
  |
4 |     42?;
  |     ^^^ trait `(): std::ops::Carrier` not satisfied
  |
  = note: required by `std::ops::Carrier::from_error`
