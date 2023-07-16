
error[E0081]: discriminant value `1` assigned more than once
  --> tst.rs:5:1
   |
5  | / enum Derp {
6  | |     First = 1,
   | |             - 1st assignment of `1`
7  | |     Second = 1,
   | |              - 2nd assignment of `1`
8  | |     Third = 1,
   | |              - 3rd assignment of `1`
9  | |     Fourth = 1,
   | |              - 4th assignment of `1`
10 | | }
   | |_^
