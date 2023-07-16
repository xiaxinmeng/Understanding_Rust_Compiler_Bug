
error[E0277]: future cannot be sent between threads safely
  --> src/main.rs:23:5
   |
23 |     is_send(foo());
   |     ^^^^^^^ future returned by `foo()` is not `Send`
   |
note: future is not send because this value is used across an await
NN |     let g = x.lock().unwrap();
   |         - has type `std::sync::MutexGuard<'_, u32>`
NN |     baz().await;
   |           ^^^^^ await occurs here, with `g` maybe used later
NN |  }
   |  - `g` is later dropped here
note: `Send` is required because of this where clause
  --> src/main.rs:5:1
   |
5  | fn is_send<T: Send>(t: T) {
   |            -------
