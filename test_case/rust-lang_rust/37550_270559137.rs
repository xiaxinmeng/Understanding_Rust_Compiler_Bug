
error[E0016]: blocks in constant functions are limited to items and tail expressions
 --> test.rs:4:13
  |
4 |     let t = true;
  |             ^^^^

error[E0016]: blocks in constant functions are limited to items and tail expressions
 --> test.rs:5:13
  |
5 |     let x = || t;
  |             ^^^^

error: aborting due to 2 previous errors
