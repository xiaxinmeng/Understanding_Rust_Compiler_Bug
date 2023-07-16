
error[E0282]: type annotations needed for the closure `fn(&_) -> i32`
 --> src/main.rs:4:16
  |
4 |   let new_el = |&_| 30;  // error: the type of this value must be known in this context
  |                ^^^^^^^ cannot infer type for `[closure@src/main.rs:4:16: 4:23]`
  |
help: give this closure an explicit return type without `_` placeholders
  |
4 |   let new_el = |&_| -> i32 { 30 };  // error: the type of this value must be known in this context
  |                     ^^^^^^^^    ^
