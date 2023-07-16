
warning: unnecessary `unsafe` block
  --> src/main.rs:10:9
   |
9  |     unsafe {
   |     ------ because it's nested under this `unsafe` block
10 |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block
   |
   = note: `#[warn(unused_unsafe)]` on by default

warning: unnecessary `unsafe` block
  --> src/main.rs:11:9
   |
9  |     unsafe {
   |     ------ because it's nested under this `unsafe` block
10 |         unsafe { unsf() }
11 |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block

warning: unnecessary `unsafe` block
  --> src/main.rs:12:9
   |
9  |     unsafe {
   |     ------ because it's nested under this `unsafe` block
...
12 |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block
