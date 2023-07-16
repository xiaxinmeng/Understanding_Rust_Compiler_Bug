
error: `derive` cannot be used on items with type macros
 --> src/main.rs:7:12
  |
7 |     field: typ!(),
  |            ^^^^^^

error[E0392]: parameter `T` is never used
 --> src/main.rs:6:17
  |
6 | struct MyStruct<T> {
  |                 ^ unused parameter
  |
  = help: consider removing `T`, referring to it in a field, or using a marker such as `std::marker::PhantomData`
