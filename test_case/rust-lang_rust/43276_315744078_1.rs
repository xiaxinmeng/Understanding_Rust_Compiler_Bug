
error[E0621]: explicit lifetime required in the type of `y`
 --> src/main.rs:2:10
  |
1 | fn foo<'a>(x:  fn(&u8, &u8), y: Vec<&u8>, z: &'a u8) {
  |                              - consider changing the type of `y` to `std::vec::Vec<&'a u8>`
2 |   y.push(z); 
  |          ^ lifetime `'a` required
