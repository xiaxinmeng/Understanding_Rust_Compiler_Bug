
error[E0308]: mismatched types
 --> src/lib.rs:8:20
  |
7 | fn bar<T>(foo: Foo) {
  |        - this type parameter
8 |     let value: T = foo.get();
  |                -   ^^^^^^^^^ expected type parameter `T`, found `u32`
  |                |
  |                expected due to this
  |
  = note: expected type parameter `T`
                       found type `u32`
