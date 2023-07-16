
error[E0603]: tuple struct `Error` is private
  --> main.rs:22:16
   |
2  |     pub struct Error(usize, pub usize, usize);
   |     ------------------------------------------ a tuple struct constructor is private if any of its fields is private
...
22 |     let x = a::Error(3, 1, 2);
   |                ^^^^^
