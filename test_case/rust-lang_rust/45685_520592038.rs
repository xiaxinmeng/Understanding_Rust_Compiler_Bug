
error[E0282]: type annotations needed for `std::option::Option<_>`
 --> src/main.rs:4:14
  |
4 |     let c = |a: Option<_>| println!("{:?}", a);
  |              ^ consider giving this closure parameter the explicit type `std::option::Option<_>`, where the type parameter `T` is specified
