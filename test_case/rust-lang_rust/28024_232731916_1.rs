 text
error: the trait bound `Foo: std::fmt::Debug` is not satisfied [--explain E0277]
 --> <anon>:9:5
9 |>     show(f);
  |>     ^^^^
note: `Foo` cannot be formatted using `:?`; if it is defined in your crate, add `#[derive(Debug)]` or manually implement it
note: required by `show`
