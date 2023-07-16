
error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
 --> src/lib.rs:3:26
  |
2 |     for i in 0..1000 {
  |         - you must specify a type for this binding, like `i32`
3 |         println!("{}", i.pow(2));
  |                          ^^^
