plain
---- src/cell.rs - cell::UnsafeCell<T>::from_mut (line 2039) stdout ----
error[E0658]: use of unstable library feature 'unsafe_cell_from_mut'
 --> src/cell.rs:2043:10
  |
7 | let uc = UnsafeCell::from_mut(&mut val);
  |
  = help: add `#![feature(unsafe_cell_from_mut)]` to the crate attributes to enable

error: aborting due to previous error
---
    src/cell.rs - cell::UnsafeCell<T>::from_mut (line 2039)

test result: FAILED. 4074 passed; 1 failed; 37 ignored; 0 measured; 0 filtered out; finished in 47.54s

error: doctest failed, to rerun pass `-p core --doc`
