
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
 --> 51972.rs:5:18
  |
4 |     for i in 0..10 {
  |         - you must specify a type for this binding, like `i32`
5 |         sum += i.pow(2);
  |                  ^^^

error: aborting due to previous error
