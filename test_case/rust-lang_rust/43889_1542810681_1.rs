shell
Compiling playground v0.0.1 (/playground)
error: future cannot be sent between threads safely
  --> src/main.rs:17:55
   |
17 |     let fut: Pin<Box<dyn Future<Output=()> + Send>> = Box::pin(get());
   |                                                       ^^^^^^^^^^^^^^^ future created by async block is not `Send`
   |
   = help: within `impl Future<Output = ()>`, the trait `Send` is not implemented for `*mut ()`
note: captured value is not `Send`
  --> src/main.rs:11:30
   |
11 |         std::hint::black_box(unsendable);
   |                              ^^^^^^^^^^ has type `Unsendable` which is not `Send`
   = note: required for the cast from `impl Future<Output = ()>` to the object type `dyn Future<Output = ()> + Send`
