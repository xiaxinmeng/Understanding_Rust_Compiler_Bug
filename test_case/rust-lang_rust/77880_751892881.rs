
error[E0282]: type annotations needed
 --> src/main.rs:5:21
  |
5 |         may_error()?;
  |                    ^ cannot infer type of `?` return
  |
  = note: The `?` operator can convert the error value to any type which implements `From<std::io::Error>`
help: consider specifying the type arguments when constructing `std::result::Result::Ok`
  |
6 |         Ok::<T, E>(())
  |           ^^^^^^^^
