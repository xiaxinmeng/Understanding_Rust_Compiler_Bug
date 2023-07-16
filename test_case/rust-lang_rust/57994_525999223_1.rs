
help: the following traits define an item `m`, perhaps you need to restrict type parameter `T` with one of them:
  |
3 |     fn foo<T: First::Foo + Foo>(a: T) {
  |            ^^^^^^^^^^^^^^^
3 |     fn foo<T: Second::Bar + Foo>(a: T) {
  |            ^^^^^^^^^^^^^^^^
