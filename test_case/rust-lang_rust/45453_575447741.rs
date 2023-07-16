
error[E0283]: type annotations needed
 --> src/main.rs:3:5
  |
1 | fn func1<T: Into<String>>(x: i32) {}
  |    -----    ------------ required by this bound in `func1`
2 | fn main() {
3 |     func1(42);
  |     ^^^^^
  |     |
  |     cannot infer type for type parameter `T` declared on the function `func1`
  |     help: consider specifying the type argument in the function call: `func1::<T>`
  |
  = note: cannot resolve `_: std::convert::Into<std::string::String>`
