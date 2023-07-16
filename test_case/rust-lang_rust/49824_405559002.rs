
error: unsatisfied lifetime constraints
  --> src/main.rs:7:9
   |
6  |         || {
   |    _____-
   |   |_____|
   |  ||
7  |  ||         || {
   |  ||_________^
8  | |||             let _y = &mut x;
9  | |||         }
   | |||_________^ free region requires that `'1` must outlive `'2`
10 |  ||     };
   |  ||     -
   |  ||_____|
   |  |______lifetime `'1` represents the closure body
   |         lifetime `'2` appears in return type
