error[E0070]: invalid left-hand side of assignment
  --> src/main.rs:40:17
   |
40 |             &*x = String::from("i changed u");
   |             --- ^
   |             |
   |             cannot assign to this expression
   |
help: you might have meant to use pattern destructuring
   |
32 |         while let cl2
   |               +++
