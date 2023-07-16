
error[E0283]: type annotations needed for `std::string::String`
 --> src/main.rs:4:18
  |
4 |         .map(|x| String::from(x.as_ref()))
  |                  ^^^^^^^^^^^^ cannot infer type for struct `std::string::String`
...
7 |     format!("Hello");
  |     ----------------- consider giving `res` a type
  |
  = note: cannot resolve `std::string::String: std::convert::From<&_>`
  = note: required by `std::convert::From::from`
