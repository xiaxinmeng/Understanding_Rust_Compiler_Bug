
   Compiling playground v0.0.1 (/playground)
error: `impl Trait` can only mention lifetimes bound at the fn or impl level
  --> src/lib.rs:16:78
   |
16 | fn foobar<T>(_: impl for<'a> MyFn<&'a T, Output = impl Future<Output = ()> + 'a>) {}
   |                                                                              ^^
   |
note: lifetime declared here
  --> src/lib.rs:16:26
   |
16 | fn foobar<T>(_: impl for<'a> MyFn<&'a T, Output = impl Future<Output = ()> + 'a>) {}
   |                          ^^

error: could not compile `playground` due to previous error
