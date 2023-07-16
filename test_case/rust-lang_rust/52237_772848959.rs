
error[E0532]: expected tuple struct or tuple variant, found struct variant `Test::Val1`
 --> src/main.rs:8:9
  |
2 |     Val1 { val: u32 },
  |     ----------------- `Test::Val1` defined here
...
8 |         Test::Val1(val) => println!("{}", val),
  |         ^^^^^^^^^^^^^^^ help: use struct pattern syntax instead: `Test::Val1 { val }`
