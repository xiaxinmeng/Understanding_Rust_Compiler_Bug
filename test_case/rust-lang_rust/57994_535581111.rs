
error[E0599]: no method named `m` found for type `T` in the current scope
 --> src/lib.rs:4:11
  |
4 |         a.m();
  |           ^ method not found in `T`
  |
  = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `m`, perhaps you need to restrict type parameter `T` with one of them:
  |
3 |     fn foo<T: First::Foo + Foo>(a: T) {
  |            ^^^^^^^^^^^^^^^
3 |     fn foo<T: Second::Bar + Foo>(a: T) {
  |            ^^^^^^^^^^^^^^^^

error[E0599]: no method named `m` found for type `impl Bar` in the current scope
  --> src/lib.rs:10:11
   |
10 |         b.m();
   |           ^ method not found in `impl Bar`
   |
   = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following traits define an item `m`, perhaps you need to restrict type parameter `impl Bar` with one of them:
   |
9  |     fn bar(b: impl Bar + First::Foo) {
   |               ^^^^^^^^^^^^^^^^^^^^^
9  |     fn bar(b: impl Bar + Second::Bar) {
   |               ^^^^^^^^^^^^^^^^^^^^^^
