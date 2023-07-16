
error[E0283]: type annotations needed
 --> src/main.rs:4:18
  |
4 |         .map(|x| String::from(x.as_ref()))
  |          ---     ^^^^^^^^^^^^ cannot infer type for struct `std::string::String`
  |          |
  |          help: consider specifying the type arguments in the method call: `map::<B, F>`
  |
  = note: cannot resolve `std::string::String: std::convert::From<&_>`
  = note: required by `std::convert::From::from`
