
error[E0308]: try expression alternatives have incompatible types
 --> src/main.rs:6:5
  |
6 |     foo()?
  |     ^^^^^-
  |     |    |
  |     |    help: try removing this `?`
  |     expected enum `std::result::Result`, found isize
  |
  = note: expected type `std::result::Result<isize, ()>`
             found type `isize`
