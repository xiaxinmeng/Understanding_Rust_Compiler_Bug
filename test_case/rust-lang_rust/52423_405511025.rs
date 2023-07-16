none
error[E0507]: cannot move out of borrowed content
  --> src/main.rs:12:11
   |
12 | fn bar<T>(&MyStruct { x: _a }: &MyStruct<T>) { }
   |           ^^^^^^^^^^^^^^^--^^
   |           |              |
   |           |              hint: to prevent move, use `ref _a` or `ref mut _a`
   |           cannot move out of borrowed content
