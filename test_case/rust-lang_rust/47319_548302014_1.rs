
9  | fn foo1<T>(t: T) {
   |         - this type parameter
10 |     bar1(t);
   |          ^
   |          |
   |          expected enum `std::option::Option`, found type parameter `T`
   |          help: try using a variant of the expected enum: `Some(t)`
