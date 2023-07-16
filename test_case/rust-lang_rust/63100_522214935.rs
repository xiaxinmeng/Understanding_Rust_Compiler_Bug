

error[E0277]: the trait bound `fn() -> impl std::future::Future {foo}: std::future::Future` is not satisfied
  --> src/main.rs:10:5
   |
10 |     bar(foo);
   |     ^^^ the trait `std::future::Future` is not implemented for `fn() -> impl std::future::Future {foo}`
   |
note: required by `bar`
  --> src/main.rs:7:1
   |
7  | fn bar(f: impl Future<Output = ()>) {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
