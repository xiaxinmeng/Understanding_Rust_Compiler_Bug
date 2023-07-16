
5 | fn bar() -> impl Generator + Send {
  |             ^^^^^^^^^^^^^^^^^^^^^ `dyn std::marker::Send` cannot be sent between threads safely
  |
  = help: the trait `for<'r> std::marker::Send` is not implemented for `dyn std::marker::Send`
  = note: required because of the requirements on the impl of `for<'r> std::marker::Send` for `std::ptr::Unique<dyn std::marker::Send>`
  = note: required because it appears within the type `std::boxed::Box<dyn std::marker::Send>`
  = note: required because it appears within the type `for<'r> {std::boxed::Box<(dyn std::marker::Send + 'r)>, ()}`
  = note: required because it appears within the type `[generator@src/main.rs:6:5: 10:6 for<'r> {std::boxed::Box<(dyn std::marker::Send + 'r)>, ()}]`
