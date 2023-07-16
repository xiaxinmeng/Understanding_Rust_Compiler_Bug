
error[E0521]: borrowed data escapes outside of closure
 --> src/main.rs:9:9
  |
6 |     let mut track = Vec::new();
  |         --------- `track` declared here, outside of the closure body
...
9 |         track.push(&arena)
  |         ^^^^^^^^^^^^^^^^^^
