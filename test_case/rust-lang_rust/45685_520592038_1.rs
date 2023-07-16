
error[E0282]: type annotations needed for `std::option::Option<T>`
 --> src/main.rs:4:14
  |
4 |     let c = |a| println!("{:?}", a);
  |              ^ consider giving this closure parameter the explicit type `std::option::Option<T>`, where the type parameter `T` is specified
