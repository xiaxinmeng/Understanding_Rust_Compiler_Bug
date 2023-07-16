
error: implementation of `Callback` is not general enough
  [--> src/lib.rs:21:5
](https://play.rust-lang.org/?version=nightly&mode=debug&edition=2021#)   |
21 |     transaction(my_callback).await;
   |     ^^^^^^^^^^^ implementation of `Callback` is not general enough
   |
   = note: `fn(&mut ()) -> impl Future<Output = ()> {my_callback::<'_>}` must implement `Callback<'0>`, for any lifetime `'0`...
   = note: ...but it actually implements `Callback<'1>`, for some specific lifetime `'1`
