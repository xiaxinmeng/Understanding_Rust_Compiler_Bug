text
error[E0505]: cannot move out of `x` because it is borrowed
  --> src/main.rs:15:5
   |
13 |     x
   |     - borrow of `x` occurs here
14 |     +=
15 |     x;
   |     ^ move out of `x` occurs here

error[E0596]: cannot borrow `y` as mutable, as it is not declared as mutable
  --> src/main.rs:18:5
   |
17 |     let y = Int(2);
   |         - help: consider changing this to be mutable: `mut y`
18 |     y
   |     ^ cannot borrow as mutable

error: aborting due to 2 previous errors
