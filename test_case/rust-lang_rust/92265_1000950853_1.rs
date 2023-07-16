
ben@Bens-MBP issue-92265 % rustc +stable src/error.rs
warning: variable does not need to be mutable
 --> src/error.rs:7:9
  |
7 |     let mut level = levels[i].get(&z).unwrap();
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[E0596]: cannot borrow `*level` as mutable, as it is behind a `&` reference
 --> src/error.rs:8:5
  |
7 |     let mut level = levels[i].get(&z).unwrap();
  |         --------- help: consider changing this to be a mutable reference: `&mut Vec<(i128, i128)>`
8 |     level.sort();
  |     ^^^^^^^^^^^^ `level` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0596`.
ben@Bens-MBP issue-92265 % rustc +beta src/error.rs
warning: variable does not need to be mutable
 --> src/error.rs:7:9
  |
7 |     let mut level = levels[i].get(&z).unwrap();
  |         ----^^^^^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default

error[E0596]: cannot borrow `*level` as mutable, as it is behind a `&` reference
 --> src/error.rs:8:5
  |
7 |     let mut level = levels[i].get(&z).unwrap();
  |         --------- help: consider changing this to be a mutable reference: `&mut Vec<(i128, i128)>`
8 |     level.sort();
  |     ^^^^^^^^^^^^ `level` is a `&` reference, so the data it refers to cannot be borrowed as mutable

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0596`.
