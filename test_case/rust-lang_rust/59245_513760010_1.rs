text
error[E0277]: `dyn std::marker::Send` cannot be shared between threads safely
  --> src/main.rs:12:5
   |
12 |     assert_send(foo(Box::new(5)));
   |     ^^^^^^^^^^^ `dyn std::marker::Send` cannot be shared between threads safely
   |
   = help: the trait `std::marker::Sync` is not implemented for `dyn std::marker::Send`
   = note: required because of the requirements on the impl of `std::marker::Sync` for `std::ptr::Unique<dyn std::marker::Send>`
   = note: required because it appears within the type `std::boxed::Box<dyn std::marker::Send>`
   = note: required because of the requirements on the impl of `std::marker::Send` for `&std::boxed::Box<dyn std::marker::Send>`
   = note: required because it appears within the type `for<'r, 's, 't0> {std::boxed::Box<(dyn std::marker::Send + 'r)>, &'s std::boxed::Box<(dyn std::marker::Send + 't0)>, impl std::future::Future, ()}`
   = note: required because it appears within the type `[static generator@src/main.rs:3:32: 7:2 x:std::boxed::Box<(dyn std::marker::Send + 'static)> for<'r, 's, 't0> {std::boxed::Box<(dyn std::marker::Send + 'r)>, &'s std::boxed::Box<(dyn std::marker::Send + 't0)>, impl std::future::Future, ()}]`
   = note: required because it appears within the type `std::future::GenFuture<[static generator@src/main.rs:3:32: 7:2 x:std::boxed::Box<(dyn std::marker::Send + 'static)> for<'r, 's, 't0> {std::boxed::Box<(dyn std::marker::Send + 'r)>, &'s std::boxed::Box<(dyn std::marker::Send + 't0)>, impl std::future::Future, ()}]>`
   = note: required because it appears within the type `impl std::future::Future`
   = note: required because it appears within the type `impl std::future::Future`
note: required by `assert_send`
  --> src/main.rs:9:1
   |
9  | fn assert_send<T: Send>(_: T) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
