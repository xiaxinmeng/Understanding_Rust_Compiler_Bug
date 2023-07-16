
error[E0507]: cannot move out of borrowed content
  --> src/lib.rs:8:11
   |
8  |     match foo.bar {
   |           ^^^^^^^
   |           |
   |           cannot move out of borrowed content
   |           help: consider borrowing here: `&foo.bar`
9  |         None => 1,
10 |         Some(s) => 2,
   |              - data moved here
   |
note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait
  --> src/lib.rs:10:14
   |
10 |         Some(s) => 2,
   |              ^
