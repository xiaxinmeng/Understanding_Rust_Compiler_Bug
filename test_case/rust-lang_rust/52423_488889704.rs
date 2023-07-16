
error[E0507]: cannot move out of borrowed content
 --> src/lib.rs:8:11
  |
8 |     match s.x {
  |           ^^^
  |           |
  |           cannot move out of borrowed content
  |           help: consider borrowing here: `&s.x`
9 |         Some(_a) => {}
  |              -- data moved here
  |
note: move occurs because `_a` has type `T`, which does not implement the `Copy` trait
 --> src/lib.rs:9:14
  |
9 |         Some(_a) => {}
  |              ^^

error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:14:11
   |
14 | fn bar<T>(&MyStruct { x: _a }: &MyStruct<T>) { }
   |           ^^^^^^^^^^^^^^^--^^
   |           |              |
   |           |              data moved here
   |           cannot move out of borrowed content
   |           help: consider removing the `&`: `MyStruct { x: _a }`
   |
note: move occurs because `_a` has type `std::option::Option<T>`, which does not implement the `Copy` trait
  --> src/lib.rs:14:26
   |
14 | fn bar<T>(&MyStruct { x: _a }: &MyStruct<T>) { }
   |                          ^^
