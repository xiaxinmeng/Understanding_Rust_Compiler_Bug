
zmd@ReflectiveCoherence:~/Code/Misc$ rustc +stage1 scratch.rs 
error[E0618]: expected function, found `()`
 --> scratch.rs:3:5
  |
3 |       foo() // missing semicolon
  |       ^    - help: try adding a semicolon: `;`
  |  _____|
  | |
4 | |
5 | |         (0, 0)
  | |______________^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0618`.
