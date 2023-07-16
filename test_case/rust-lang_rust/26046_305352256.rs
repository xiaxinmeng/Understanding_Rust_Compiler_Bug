
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> test.rs:17:19
   |
17 |       let closure = move || {
   |  ___________________^
18 | |         vec
19 | |     };
   | |_____^
   |
note: the requirement to implement `Fn` derives from here
  --> test.rs:21:5
   |
21 |     Box::new(closure)
   |     ^^^^^^^^^^^^^^^^^

error: aborting due to previous error(s)
