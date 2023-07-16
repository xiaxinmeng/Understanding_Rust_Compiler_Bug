
note: move occurs because `a` has type `std::string::String` and `b` has type `std::string::String`, which do not implement the `Copy` trait
  --> a.rs:13:14
   |
13 |     let &X { a, b } = &x;
   |              ^  ^
