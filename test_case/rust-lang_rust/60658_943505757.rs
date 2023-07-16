
error[E0277]: `(dyn Future<Output = ()> + Send + 'static)` cannot be unpinned
  --> src/main.rs:16:5
   |
16 |     type Future = Box<dyn Future<Output = ()> + Send>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Unpin` is not implemented for `(dyn Future<Output = ()> + Send + 'static)`
   |
   = note: consider using `Box::pin`
   = note: required because of the requirements on the impl of `Future` for `Box<(dyn Future<Output = ()> + Send + 'static)>`
note: required by a bound in `Foo::Future`
  --> src/main.rs:4:25
   |
4  |     type Future: Future<Output = ()>;
   |                         ^^^^^^^^^^^ required by this bound in `Foo::Future`

error: implementation of `Send` is not general enough
  --> src/main.rs:19:9
   |
19 | /         Box::new(async move {
20 | |             T::foo().await
21 | |         })
   | |__________^ implementation of `Send` is not general enough
   |
   = note: `<T as Foo<'0>>::Future` must implement `Send`, for any lifetime `'0`...
   = note: ...but `Send` is actually implemented for the type `<T as Foo<'a>>::Future`
