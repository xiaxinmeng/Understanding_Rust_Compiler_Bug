
warning: type `test` should have an upper camel case name
 --> test2.rs:1:8
  |
1 | struct test {
  |        ^^^^ help: convert the identifier to upper camel case: `Test`
  |
  = note: `#[warn(non_camel_case_types)]` on by default

warning: unused variable: `x`
 --> test2.rs:5:9
  |
5 | fn test(x: i32) {
  |         ^ help: consider prefixing with an underscore: `_x`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
  --> test2.rs:10:9
   |
10 |     let x = test(1);
   |         ^ help: consider prefixing with an underscore: `_x`

warning: struct is never constructed: `test`
 --> test2.rs:1:8
  |
1 | struct test {
  |        ^^^^
  |
  = note: `#[warn(dead_code)]` on by default
