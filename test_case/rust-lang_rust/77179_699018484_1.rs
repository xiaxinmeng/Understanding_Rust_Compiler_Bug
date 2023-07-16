
error: non-defining opaque type use in defining scope
 --> gat.rs:5:14
  |
5 | fn test() -> Pointer<i32> {
  |              ^^^^^^^^^^^^
  |
note: used non-generic type `i32` for generic parameter
 --> gat.rs:3:14
  |
3 | type Pointer<T> = impl std::ops::Deref<Target=T>;
  |              ^

error: aborting due to previous error
