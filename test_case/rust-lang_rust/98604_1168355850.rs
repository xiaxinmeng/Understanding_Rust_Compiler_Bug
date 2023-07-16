
error[[E0271]](https://doc.rust-lang.org/nightly/error-index.html#E0271): type mismatch resolving `<fn() -> impl Future<Output = ()> {test} as FnOnce<()>>::Output == Pin<Box<dyn Future<Output = ()>>>`
 --> src/main.rs:5:5
  |
5 |     Box::new(test) as Box<
  |     ^^^^^^^^^^^^^^ expected struct `Pin`, found opaque type
  |
note: while checking the return type of the `async fn`
 --> src/main.rs:1:17
  |
1 | async fn test() {}
  |                 ^ checked the `Output` of this `async fn`, found opaque type
  = note:   expected struct `Pin<Box<dyn Future<Output = ()>>>`
          found opaque type `impl Future<Output = ()>`
  = note: required for the cast to the object type `dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>`

For more information about this error, try `rustc --explain E0271`.
error: could not compile `playground` due to previous error
