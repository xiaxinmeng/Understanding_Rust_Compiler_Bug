
error[E0507]: cannot move out of `other.ptr` which is behind a shared reference
  --> src/main.rs:17:21
   |
17 |         self.ptr == other.ptr
   |                     ^^^^^^^^^ move occurs because `other.ptr` has type `std::boxed::Box<Foo<dyn Trait>>`, which does not implement the `Copy` trait
