
error[[E0282]](https://doc.rust-lang.org/stable/error-index.html#E0282): type annotations needed for `Foo<N>`
 --> src/main.rs:8:9
  |
8 |     let foo = Foo::new();
  |         ^^^
  |
help: consider giving `foo` an explicit type, where the the value of const parameter `N` is specified
  |
8 |     let foo: Foo<N> = Foo::new();
  |            ++++++++

For more information about this error, try `rustc --explain E0282`.
error: could not compile `playground` due to previous error
