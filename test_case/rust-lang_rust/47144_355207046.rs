
error: borrowed data cannot be moved outside of its closure
  --> ../../src/test/compile-fail/issue-7573.rs:31:27
   |
27 |     let mut lines_to_use: Vec<&CrateId> = Vec::new();
   |                               - cannot infer an appropriate lifetime
28 |
29 |     let push_id = |installed_id: &CrateId| {
   |                   ------------------------ borrowed data cannot outlive this closure
30 |
31 |         lines_to_use.push(installed_id);
   |                           ^^^^^^^^^^^^ cannot be moved outside of its closure
