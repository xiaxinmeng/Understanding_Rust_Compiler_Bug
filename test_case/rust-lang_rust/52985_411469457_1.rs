
./dev/code/rust$ rustc +stage1 ./dev/tests/data/rust_52895_minimal_ice.rs 
warning: unreachable expression
 --> ./dev/tests/data/rust_52895_minimal_ice.rs:9:13
  |
9 |     let x = bar(unimplemented!());
  |             ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: #[warn(unreachable_code)] on by default

warning: unused variable: `x`
 --> ./dev/tests/data/rust_52895_minimal_ice.rs:9:9
  |
9 |     let x = bar(unimplemented!());
  |         ^ help: consider using `_x` instead
  |
  = note: #[warn(unused_variables)] on by default
