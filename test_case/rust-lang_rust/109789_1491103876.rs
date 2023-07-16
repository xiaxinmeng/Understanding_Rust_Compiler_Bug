
error: implementation of `Mid` is not general enough
  --> src/main.rs:17:11
   |
17 | fn bar(_: <Foo<for<'a> fn(&'a ())> as Mid>::Assoc) {}
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `Mid` is not general enough
   |
   = note: `Foo<for<'a> fn(&'a ())>` must implement `Mid`
   = note: ...but `Mid` is actually implemented for the type `Foo<fn(&'static ())>`
