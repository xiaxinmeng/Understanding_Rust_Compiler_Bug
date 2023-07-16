
error[E0267]: `break` inside of a closure
  --> $DIR/E0267.rs:5:18
   |
LL |     let _ = || { break; };
   |             --   ^^^^^ cannot `break` inside of a closure
   |             |
   |             enclosing closure
