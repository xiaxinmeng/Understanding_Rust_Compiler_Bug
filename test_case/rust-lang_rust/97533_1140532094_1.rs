
error[E0081]: discriminant value `1` assigned more than once
  --> tst.rs:5:1
   |
5  | / enum Derp {
6  | |     First = 1,
   | |             - first assignment of `1`
7  | |     Second = 1,
   | |              - second assignment of `1`
8  | |     Third = 1,
9  | |     Fourth = 1,
10 | | }
   | |_^

error[E0081]: discriminant value `1` assigned more than once
  --> tst.rs:5:1
   |
5  | / enum Derp {
6  | |     First = 1,
   | |             - first assignment of `1`
7  | |     Second = 1,
8  | |     Third = 1,
   | |             - second assignment of `1`
9  | |     Fourth = 1,
10 | | }
   | |_^

error[E0081]: discriminant value `1` assigned more than once
  --> tst.rs:5:1
   |
5  | / enum Derp {
6  | |     First = 1,
   | |             - first assignment of `1`
7  | |     Second = 1,
8  | |     Third = 1,
9  | |     Fourth = 1,
   | |              - second assignment of `1`
10 | | }
   | |_^

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0081`.
