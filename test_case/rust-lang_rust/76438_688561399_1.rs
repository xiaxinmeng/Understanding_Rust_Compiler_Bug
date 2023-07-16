
error[E0308]: mismatched types
 --> src/lib.rs:2:33
  |
1 | fn f<Amount: std::ops::Sub<Output = TotalAmount>, TotalAmount>(amount: Amount, total_amount: TotalAmount) {
  |      ------ expected type parameter               ----------- found type parameter
2 |     let total_amount = amount - total_amount;
  |                                 ^^^^^^^^^^^^ expected type parameter `Amount`, found type parameter `TotalAmount`
  |
  = note: expected type parameter `Amount`
             found type parameter `TotalAmount`
help: consider further restricting this bound
  |
1 | fn f<Amount: std::ops::Sub<TotalAmount, Output = TotalAmount>, TotalAmount>(amount: Amount, total_amount: TotalAmount) {
  |                            ^^^^^^^^^^^^
