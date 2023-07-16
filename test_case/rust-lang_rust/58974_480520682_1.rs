console
error[E0277]: the trait bound `E: std::error::NotBoxDynError` is not satisfied in `std::error::IsBoxDynError<E>`
 --> src/main.rs:5:7
  |
5 |     e.into()
  |       ^^^^ within `std::error::IsBoxDynError<E>`, the trait `std::error::NotBoxDynError` is not implemented for `E`
  |
  = help: consider adding a `where E: std::error::NotBoxDynError` bound
  = note: required because it appears within the type `std::error::IsBoxDynError<E>`
  = note: required because of the requirements on the impl of `std::convert::From<E>` for `std::boxed::Box<dyn std::error::Error>`
  = note: required because of the requirements on the impl of `std::convert::Into<std::boxed::Box<dyn std::error::Error>>` for `E`
