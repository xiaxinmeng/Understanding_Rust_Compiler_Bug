
error[E0308]: mismatched types
  --> analog.rs:17:11
   |
17 | fn bar(_: <Foo<for<'a> fn(&'a ())> as Mid>::Assoc) {}
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `<Foo<for<'a> fn(&'a ())> as Mid>`
              found trait `<Foo<fn(&'static ())> as Mid>`
