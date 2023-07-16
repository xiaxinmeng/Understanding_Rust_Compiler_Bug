
error: future cannot be sent between threads safely
  --> src/main.rs:17:5
   |
6  | fn require_send(_: impl Send) {}
   |    ------------         ---- required by this bound in `require_send`
...
17 |     require_send(send_fut);
   |     ^^^^^^^^^^^^ future returned by `main` is not `Send`
   |
   = help: the trait `std::marker::Sync` is not implemented for `std::cell::RefCell<i32>`
note: future is not `Send` as this value is used across an await
  --> src/main.rs:15:17
   |
14 |         let non_send_fut = make_non_send_future();
   |             ------------ has type `impl core::future::future::Future`
15 |         let _ = non_send_fut.await;
   |                 ^^^^^^^^^^^^^^^^^^ await occurs here, with `non_send_fut` maybe used later
16 |         //ready(0).await;
17 |     };
   |     - `non_send_fut` is later dropped here
