
error[E0277]: `Vec<{integer}>` doesn't implement `std::fmt::Display`
  --> src/main.rs:21:14
   |
21 |     Foo::foo(&vec);
   |     -------- ^^^^ `Vec<{integer}>` cannot be formatted with the default formatter
   |     |
   |     required by a bound introduced by this call
   |
   = help: the trait `std::fmt::Display` is not implemented for `Vec<{integer}>`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required for `Vec<{integer}>` to implement `Foo`
  --> src/main.rs:13:18
   |
13 | impl<T: Display> Foo for T {
   |                  ^^^     ^
