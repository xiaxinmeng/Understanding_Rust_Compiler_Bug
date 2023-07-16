text
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
 --> <anon>:4:19
  |
4 |     let closure = move || {
  |                   ^
  |
note: the requirement to implement `Fn` derives from here
 --> <anon>:8:5
  |
8 |     Box::new(closure)
  |     ^^^^^^^^^^^^^^^^^
