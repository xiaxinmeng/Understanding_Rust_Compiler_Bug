 bash
./x86_64-unknown-linux-gnu/stage1/bin/rustc src/test/compile-fail/E0061.rs
error[E0061]: this function takes 2 parameters but 1 parameter was supplied
  --> src/test/compile-fail/E0061.rs:14:5
   |
14 |     f(0);
   |     ^^^^
   |
   = note: the following parameter types were expected: 
* u16
* &str

error: aborting due to previous error
