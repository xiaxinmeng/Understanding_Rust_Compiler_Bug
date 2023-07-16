console
error[E0271]: type mismatch resolving `<fn() -> impl Future<Output = ()> {test} as FnOnce<()>>::Output == Pin<Box<(dyn Future<Output = ()> + 'static)>>`
 --> src/main.rs:9:5
  |
9 |     Box::new(test) as AsyncFnPtr;
  |     ^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
  |
note: while checking the return type of the `async fn`
 --> src/main.rs:5:17
  |
5 | async fn test() {}
  |                 ^ checked the `Output` of this `async fn`, found opaque type
  = note:   expected struct `Pin<Box<(dyn Future<Output = ()> + 'static)>>`
          found opaque type `impl Future<Output = ()>`
  = note: required for the cast to the object type `dyn Fn() -> Pin<Box<(dyn Future<Output = ()> + 'static)>>`

For more information about this error, try `rustc --explain E0271`.
