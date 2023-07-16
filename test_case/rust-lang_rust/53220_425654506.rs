text
error[E0713]: borrow may still be in use when destructor runs
  --> <source>:50:39
   |
50 |             Ok(child) => members.push(child.raw),
   |                                       ^^^^^^^^^
51 |             Err(_) => ()
52 |         }
   |         - here, drop of `child` needs exclusive access to `*child.raw`, because the type `Child<'_>` implements the `Drop` trait
53 |     }
54 |     members.len();
   |     ------- borrow used here in later iteration of loop
error: aborting due to previous error
For more information about this error, try `rustc --explain E0713`.
Compiler returned: 1
