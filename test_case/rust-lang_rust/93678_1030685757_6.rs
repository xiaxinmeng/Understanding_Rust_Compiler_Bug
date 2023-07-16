
warning: unnecessary `unsafe` block
 --> src/main.rs:9:16
  |
9 |     let _ = || unsafe {
  |                ^^^^^^ unnecessary `unsafe` block
  |
  = note: `#[warn(unused_unsafe)]` on by default

warning: unnecessary `unsafe` block
  --> src/main.rs:10:20
   |
10 |         let _ = || unsafe { unsf() };
   |                    ^^^^^^ unnecessary `unsafe` block
