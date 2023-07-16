
error[E0308]: mismatched types
 --> src/lib.rs:2:33
  |
1 | fn f<Amount: std::ops::Sub, TotalAmount>(amount: Amount, total_amount: TotalAmount) {
  |      ------                 ----------- found type parameter
  |      |
  |      expected type parameter
2 |     let total_amount = amount - total_amount;
  |                                 ^^^^^^^^^^^^ expected type parameter `Amount`, found type parameter `TotalAmount`
  |
  = note: expected type parameter `Amount`
             found type parameter `TotalAmount`
  = note: a type parameter was expected, but a different one was found; you might be missing a type parameter or trait bound
  = note: for more information, visit https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters
