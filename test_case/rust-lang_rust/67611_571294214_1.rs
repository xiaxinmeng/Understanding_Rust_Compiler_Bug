
error: future cannot be sent between threads safely
  --> src/lib.rs:65:5
   |
56 | pub fn spawn<T>(_task: T) -> JoinHandle<T::Output>
   |        -----
57 | where
58 |     T: Future + Send + 'static,
   |                 ---- required by this bound in `spawn`
...
65 |     spawn(proxy());
   |     ^^^^^ future returned by `proxy` is not `Send`
   |
   = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `*mut u64`
note: future is not `Send` as this value is used across an await
  --> src/lib.rs:70:5
   |
70 | /     timeout
71 | |         (Duration::from_millis(unsafe { TIMEOUT }),
   | |                                         ------- has type `*mut u64`
72 | |          async { Result::Ok(vec![0u8]) })
73 | |         .await?
   | |______________^ await occurs here, with `TIMEOUT` maybe used later
74 |           .unwrap()
75 |           ;
   |           - `TIMEOUT` is later dropped here

error: aborting due to previous error
