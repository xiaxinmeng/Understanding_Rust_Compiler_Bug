
error[E0277]: `B` doesn't implement `std::fmt::Debug`
  --> src/main.rs:65:22
   |
64 | fn foo<B: Bar>(f: Foo<B>) {
   |        ------ help: add a `Debug` bound: `B: Bar + std::fmt::Debug`
65 |     println!("{:?}", f);
   |                      ^ `B` cannot be formatted using `:?` because it doesn't implement `std::fmt::Debug`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `B`
   = note: required because of the requirements on the impl of `std::fmt::Debug` for `middlewares::authentication::Foo<B>`
   = note: required by `std::fmt::Debug::fmt
