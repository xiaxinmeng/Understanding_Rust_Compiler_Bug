
error[E0594]: cannot assign to immutable field `t.v`
  --> diag.rs:13:9
   |
13 |         t.v += 1;
   |         ^^^^^^^^ cannot mutably borrow immutable field

error: aborting due to previous error
