
warning: trait-associated function `try_from` will become ambiguous in Rust 2021
  --> src/main.rs:16:5
   |
16 |     Foo::try_from(todo!());
   |     ^^^^^^^^^^^^^ help: disambiguate the associated function: `<Foo<_> as A>::try_from`
   |
