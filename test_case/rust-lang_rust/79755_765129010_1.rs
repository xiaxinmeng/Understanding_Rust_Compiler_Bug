
error[E0308]: `if` and `else` have incompatible types
  --> src/main.rs:6:12
   |
4  |        let val = if true {
   |   _______________-
5  |  |         *a
   |  |         -- expected because of this
6  |  |     } else if true {
   |  |____________^
7  | ||         b
8  | ||     } else {
9  | ||         &0
10 | ||     };
   | ||     ^
   | ||_____|
   | |______`if` and `else` have incompatible types
   |        expected integer, found `&{integer}`
   |
help: consider dereferencing the borrow
   |
6  |     } else *if true {
7  |         b
8  |     } else {
9  |         &0
10 |     };
   |
