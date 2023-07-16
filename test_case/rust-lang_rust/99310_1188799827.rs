
error: future cannot be sent between threads safely
 --> src/main.rs:4:53
  |
4 |     let task: Box<dyn Future<Output = ()> + Send> = Box::new(async {
  |                                                     ^^^^^^^^^^^^^^ future created by async block is not `Send`
  |
  = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `Rc<i32>`
note: future is not `Send` as this value is used across an await
 --> src/main.rs:6:18
  |
5 |         let rc = std::rc::Rc::new(1);
  |             -- has type `Rc<i32>` which is not `Send`
6 |         ready(()).await
  |                  ^^^^^^ await occurs here, with `rc` maybe used later
7 |     });
  |     - `rc` is later dropped here
  = note: required for the cast to the object type `dyn Future<Output = ()> + Send`
