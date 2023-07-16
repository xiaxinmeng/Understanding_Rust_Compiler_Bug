
error[E0713]: borrow may still be in use when destructor runs
  --> src/main.rs:24:22
   |
24 |         members.push(child.raw);
   |                      ^^^^^^^^^
25 |     }
   |     - here, drop of `child` needs exclusive access to `*child.raw`, because the type `C<'_>` implements the `Drop` trait
26 |     members.len();
   |     ------- borrow used here, in later iteration of loop

error: aborting due to previous error
