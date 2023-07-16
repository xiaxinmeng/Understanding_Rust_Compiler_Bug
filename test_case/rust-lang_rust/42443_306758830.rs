
  --> src/test/ui/closure_context/issue-26046-fn-once.rs:14:19
   |
14 |       let closure = move || {
   |  ___________________^
15 | |         vec
16 | |     };
   | |_____^
17 | 
18 |       Box::new(closure)
   |       ----------------- the requirement to implement `Fn` derives from here
   |
note: closure is `FnOnce` because it moves the variable `vec` out of its environment
  --> src/test/ui/closure_context/issue-26046-fn-once.rs:15:9
   |
15 |         vec
   |         ^^^

error: aborting due to previous error(s)
