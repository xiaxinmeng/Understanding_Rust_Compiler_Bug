
error[E0081]: discriminant value `0` assigned more than once
  --> tst.rs:5:1
   |
5  | enum MyEnum {
   | ^^^^^^^^^^^
6  |     V0 = 0,
   |          - `0` assigned here
7  |     V1 = -2,
   |     ------- discriminant for `V3` incremented from this startpoint (`V1` + 2 variant later => `V3` = 0)
8  |     V2,
9  |     V3,
   |     -- `0` assigned here
10 |     V4 = 0,
   |          - `0` assigned here

error: aborting due to previous error
