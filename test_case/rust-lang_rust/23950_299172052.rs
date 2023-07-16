
error[E0277]: the trait bound `std::error::Error + 'static: std::marker::Send` is not satisfied
  --> ./test.rs:22:13
   |
22 |     let t = scoped(move|| {
   |             ^^^^^^ the trait `std::marker::Send` is not implemented for `std::error::Error + 'static`
   |
   = note: `std::error::Error + 'static` cannot be sent between threads safely
   = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<std::error::Error + 'static>`
   = note: required because it appears within the type `std::boxed::Box<std::error::Error + 'static>`
   = note: required because it appears within the type `A`
   = note: required because of the requirements on the impl of `std::marker::Send` for `std::sync::mpsc::Sender<A>`
   = note: required because it appears within the type `[closure@./test.rs:22:20: 24:6 tx:std::sync::mpsc::Sender<A>]`
   = note: required by `scoped`

error: aborting due to previous error
