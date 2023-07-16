
error[E0277]: `B` doesn't implement `std::fmt::Debug`
  --> src/main.rs:65:22
   |
65 |     println!("{:?}", f);
   |                      ^ `B` cannot be formatted using `:?` because it doesn't implement `std::fmt::Debug`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `B`
   = help: consider adding a `where B: std::fmt::Debug` bound
   = note: required because of the requirements on the impl of `std::fmt::Debug` for `middlewares::authentication::Foo<B>`
   = note: required by `std::fmt::Debug::fmt
