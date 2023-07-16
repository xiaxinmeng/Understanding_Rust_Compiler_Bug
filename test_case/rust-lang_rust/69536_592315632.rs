rust
error[E0381]: borrow of possibly-uninitialized variable: `bytes`
 --> src/lib.rs:3:13
  |
3 |         bar(&mut bytes);
  |             ^^^^^^^^^^ use of possibly-uninitialized `bytes`
