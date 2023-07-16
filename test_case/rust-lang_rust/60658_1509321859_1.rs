
error: lifetime may not live long enough
  --> src/lib.rs:21:9
   |
12 |   impl<'a, T> Foo<'a> for MyType<T>
   |        -- lifetime `'a` defined here
...
21 | /         Box::pin(async move {
22 | |             T::foo().await
23 | |         })
   | |__________^ cast requires that `'a` must outlive `'static`

error: higher-ranked lifetime error
  --> src/lib.rs:21:9
   |
21 | /         Box::pin(async move {
22 | |             T::foo().await
23 | |         })
   | |__________^
   |
   = note: could not prove `Pin<Box<[async block@src/lib.rs:21:18: 23:10]>>: CoerceUnsized<Pin<Box<(dyn Future<Output = ()> + Send + 'b)>>>`

error: could not compile `playground` due to 2 previous errors
