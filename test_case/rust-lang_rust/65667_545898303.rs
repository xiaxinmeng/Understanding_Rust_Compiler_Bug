
error: future cannot be sent between threads safely
   --> src/main.rs:16:5
    |
16  |     tokio::spawn(wait());
    |     ^^^^^^^^^^^^ future returned by `wait` is not `Send`
    |
   ::: /home/david/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-0.2.0-alpha.6/src/executor.rs:100:40
    |
100 |     F: Future<Output = ()> + 'static + Send,
    |                                        ---- required by this bound in `tokio::executor::spawn`
    |
    = help: within `impl std::future::Future`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<i32>`
note: future is not `Send` as this value is used across an await
   --> src/main.rs:9:5
    |
7   |     let rc = Rc::new(0);
    |         -- has type `std::rc::Rc<i32>`
8   |     println!("Before sleep");
9   |     delay(tokio::clock::now() + Duration::from_millis(500)).await;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `rc` maybe used later
10  |     println!("After sleep");
11  | }
    | - `rc` is later dropped here

error: aborting due to previous error
