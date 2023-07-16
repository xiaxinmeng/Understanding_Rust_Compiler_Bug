
   Compiling playground v0.0.1 (/playground)
error[E0382]: borrow of moved value: `v`
  --> src/main.rs:13:25
   |
6  |     let v = V(123);
   |         - move occurs because `v` has type `V`, which does not implement the `Copy` trait
7  |     
8  |     a.map(|x| {
   |           --- value moved into closure here
9  |         let _ = x + v.0;
   |                     - variable moved due to use in closure
...
13 |     println!("v: {:?}", v);
   |                         ^ value borrowed here after move

error: aborting due to previous error

For more information about this error, try `rustc --explain E0382`.
error: Could not compile `playground`.

To learn more, run the command again with --verbose.
