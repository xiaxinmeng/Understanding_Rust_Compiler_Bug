
% rustc src/test/compile-fail/issue-7573.rs
error: borrowed data cannot be moved outside of its closure
  --> src/test/compile-fail/issue-7573.rs:27:31
   |
27 |     let mut lines_to_use: Vec<&CrateId> = Vec::new();
   |                               ^ cannot be moved outside of its closure
28 |
29 |     let push_id = |installed_id: &CrateId| {
   |         -------   ------------------------ ...because it cannot outlive this closure
   |         |
   |         borrowed data cannot be moved into here...

error: aborting due to previous error
