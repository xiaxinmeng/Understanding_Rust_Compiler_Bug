
error[E0369]: binary operation `==` cannot be applied to type `B`
  --> src/main.rs:70:15
   |
70 |     let _ = b == a;
   |             - ^^ - A
   |             |
   |             B
   |
   = note: an implementation of `std::cmp::PartialEq` is available for `A`, try `let _ = a == b;`
