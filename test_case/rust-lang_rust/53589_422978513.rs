
error[E0392]: parameter `U` is never used
 --> src/main.rs:2:18
  |
2 | struct Cacher<T, U: Copy>
  |                  ^ unused type parameter
  |
  = help: consider removing `U`
  |
2 | struct Cacher<T>
  |               --
  = help: alternatively, consider using a marker such as `std::marker::PhantomData`
  |
7 |     __marker: std::marker::PhantomData<U>,
  |
