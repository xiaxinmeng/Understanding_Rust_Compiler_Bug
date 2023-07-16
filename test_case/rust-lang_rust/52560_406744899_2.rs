
error[E0277]: `B` doesn't implement `std::fmt::Debug`
  --> file9.rs:12:22
   |
11 | fn foo<B: Bar>(f: Foo<B>) {
   |        -- help: consider further restricting this bound: `B: std::fmt::Debug +`
12 |     println!("{:?}", f);
   |                      ^ `B` cannot be formatted using `{:?}` because it doesn't implement `std::fmt::Debug`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `B`
   = note: required because of the requirements on the impl of `std::fmt::Debug` for `Foo<B>`
   = note: required by `std::fmt::Debug::fmt`
