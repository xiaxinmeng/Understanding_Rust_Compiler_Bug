
error[E0532]: expected tuple struct/variant, found struct variant `Test::Val1`
 --> src/lib.rs:8:9
  |
2 |     Val1 { val: u32 },
  |     ----------------- `Test::Val1` defined here
...
8 |         Test::Val1(val) => println!("{}", val),
  |         ^^^^^^^^^^ did you mean `Test::Val1 { /* fields */ }`?
