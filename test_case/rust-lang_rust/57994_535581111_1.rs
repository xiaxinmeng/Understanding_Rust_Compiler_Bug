
error[E0599]: no method named `test` found for type `impl Test` in the current scope
 --> src/lib.rs:8:7
  |
8 |     t.test()
  |       ^^^^ method not found in `impl Test`
  |
  = help: items from traits can only be used if the type parameter is bounded by the trait
help: the following trait defines an item `test`, perhaps you need to restrict type parameter `impl Test` with it:
  |
7 | fn foo(t: impl Test + Test) {
  |           ^^^^^^^^^^^^^^^^
