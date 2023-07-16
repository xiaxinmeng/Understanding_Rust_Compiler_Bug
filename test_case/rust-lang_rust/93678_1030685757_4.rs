
warning: unnecessary `unsafe` block
  --> src/main.rs:10:9
   |
9  |     unsafe {
   |     ------ because it's nested under this `unsafe` block
10 |         unsafe { unsf() }
   |         ^^^^^^ unnecessary `unsafe` block
   |
   = note: `#[warn(unused_unsafe)]` on by default
