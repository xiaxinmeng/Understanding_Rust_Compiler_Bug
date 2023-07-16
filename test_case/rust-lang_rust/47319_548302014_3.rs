
17 | impl<T> S<T> {
   |      - this type parameter
18 |     fn foo2(self) {
19 |         bar1(self.t);
   |              ^^^^^^
   |              |
   |              expected enum `std::option::Option`, found type parameter `T`
   |              help: try using a variant of the expected enum: `Some(self.t)`
