
error[E0621]: explicit lifetime required in the type of `y`
 --> file.rs:2:12
  |
1 | fn foo<'a>(x: &'a i32, y: Vec<&i32>) {
  |                           --------- help: add explicit lifetime `'a` to the type of `y`: `std::vec::Vec<&'a i32>`
2 |     y.push(x);
  |            ^ lifetime `'a` required
