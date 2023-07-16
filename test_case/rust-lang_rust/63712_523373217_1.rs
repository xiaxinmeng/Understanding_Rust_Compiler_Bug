
error[E0267]: `break` inside of an `async` closure
  --> $DIR/E0267.rs:7:24
   |
LL |     let _ = async || { break; };
   |                      --^^^^^---
   |                      | |
   |                      | cannot `break` inside of an `async` closure
   |                      enclosing `async` closure
