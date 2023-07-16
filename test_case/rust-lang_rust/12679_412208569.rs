
error[E0282]: type annotations needed
 --> src/main.rs:4:16
  |
4 |   let new_el = |&_| 30;  // error: the type of this value must be known in this context
  |       ------   ^^^^^^^ cannot infer type for `[closure@src/main.rs:4:16: 4:23]`
  |       |
  |       consider giving `new_el` a type
