
error[E0308]: mismatched types
 --> src\main.rs:5:5
  |
5 |     enter(run())
  |     ^^^^^ one type is more general than the other
  |
  = note: expected type `std::ops::FnOnce<(std::iter::Map<std::vec::IntoIter<()>, [closure@src\main.rs:11:51: 11:56 x:&()]>,)>`
             found type `std::ops::FnOnce<(std::iter::Map<std::vec::IntoIter<()>, [closure@src\main.rs:11:51: 11:56 x:&()]>,)>`
