
  |
4 |     let x = (Ok(1), Ok("hi"));
  |         -    ^^ cannot infer type for type parameter `E` declared on the enum `Result`
  |         |
  |         consider giving `x` the explicit type `(std::result::Result<i32, E>, std::result::Result<&str, E>)`, where the type parameter `E` is specified
