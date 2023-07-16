
error[E0392]: parameter `T` is never used
 --> src/main.rs:1:12
  |
1 | struct Foo<T>;
  |            ^ unused type parameter
  |
  = help: consider removing `T` or using a marker such as `std::marker::PhantomData`
