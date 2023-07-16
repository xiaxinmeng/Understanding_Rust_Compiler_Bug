
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
  --> src/main.rs:16:31
   |
16 |     let mut lines_to_use: Vec<&CrateId> = Vec::new(); //~ ERROR E0495
   |                               ^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the body at 17:19...
  --> src/main.rs:17:19
   |
17 |       let push_id = |installed_id: &CrateId| {
   |  ___________________^
18 | |         lines_to_use.push(installed_id);
19 | |     };
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:18:27
   |
18 |         lines_to_use.push(installed_id);
   |                           ^^^^^^^^^^^^
note: but, the lifetime must be valid for the block suffix following statement 1 at 17:5...
  --> src/main.rs:17:5
   |
17 | /     let push_id = |installed_id: &CrateId| {
18 | |         lines_to_use.push(installed_id);
19 | |     };
20 | |     list_database(push_id);
...  |
25 | |
26 | | }
   | |_^
note: ...so that variable is valid at time of its declaration
  --> src/main.rs:17:9
   |
17 |     let push_id = |installed_id: &CrateId| {
   |         ^^^^^^^

error: aborting due to previous error
