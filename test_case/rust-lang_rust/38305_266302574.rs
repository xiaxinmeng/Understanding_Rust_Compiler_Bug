rust
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> <anon>:31:26
   |
31 |     let f = Wrapper { f: || foo };
   |                          ^^^^^^
   |
note: the requirement to implement `Fn` derives from here
  --> <anon>:32:5
   |
32 |     f();
   |     ^^^
