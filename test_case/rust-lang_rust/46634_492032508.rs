
error[E0382]: assign of moved value: `t`
  --> src/main.rs:28:5
   |
25 |     let mut t = Test2 { b: None };
   |         ----- move occurs because `t` has type `Test2`, which does not implement the `Copy` trait
26 |     let u = Test;
27 |     drop(t);
   |          - value moved here
28 |     t.b = Some(u);
   |     ^^^ value assigned here after move

error[E0382]: assign of moved value: `t`
  --> src/main.rs:34:5
   |
31 |     let mut t = Test3(None);
   |         ----- move occurs because `t` has type `Test3`, which does not implement the `Copy` trait
32 |     let u = Test;
33 |     drop(t);
   |          - value moved here
34 |     t.0 = Some(u);
   |     ^^^ value assigned here after move

error: aborting due to 2 previous errors
