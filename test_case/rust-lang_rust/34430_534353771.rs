text
   Compiling playground v0.0.1 (/playground)
error[E0271]: type mismatch resolving `for<'a> <[closure@src/main.rs:20:22: 20:31] as std::ops::FnOnce<(&'a Foo,)>>::Output == <FooRef as WithLifetime<'a>>::Type`
  --> src/main.rs:20:5
   |
20 |     wub::<FooRef, _>(|foo| foo)
   |     ^^^^^^^^^^^^^^^^ expected &Foo, found associated type
   |
   = note: expected type `&Foo`
              found type `<FooRef as WithLifetime<'_>>::Type`
note: required by `wub`
  --> src/main.rs:13:1
   |
13 | / fn wub<T, F>(f: F)
14 | |     where T: for<'a> WithLifetime<'a>,
15 | |           F: for<'a> FnOnce(&'a Foo) -> <T as WithLifetime<'a>>::Type
16 | | {
17 | | }
   | |_^

error: aborting due to previous error
