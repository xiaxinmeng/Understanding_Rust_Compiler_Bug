
$ rustc 25993.rs 
25993.rs:3:5: 3:14 error: duplicate definition of type or module `Bar`
25993.rs:3     fn Bar();
               ^~~~~~~~~
25993.rs:2:5: 2:14 note: first definition of type or module `Bar` here
25993.rs:2     type Bar;
               ^~~~~~~~~
error: aborting due to previous error
