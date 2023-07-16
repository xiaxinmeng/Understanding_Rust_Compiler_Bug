plain
..........i.......................i..................................................... 3784/3825
.........................................
failures:

---- src/num/mod.rs - num::u8::to_ascii_digit (line 342) stdout ----
error[E0425]: cannot find value `fifteen_hex` in this scope
  |
  |
9 | assert_eq!(fifteen_hex.to_ascii_digit(16), Some(15));


error[E0689]: can't call method `to_ascii_digit` on ambiguous numeric type `{integer}`
  |
5 | let one = 1;
5 | let one = 1;
  |     --- you must specify a type for this binding, like `i32`
...
8 | assert_eq!(one.to_ascii_digit(10), Some(1));

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0425, E0689.
