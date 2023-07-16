
error[E0623]: lifetime mismatch
 --> kk.rs:4:10
  |
3 | fn foo(x:fn(Ref, Ref), y: Vec<&u8>, z: &u8) {
  |             ---  --- these two types are declared with different lifetimes...
4 |   y.push(z);
  |          ^ ...but data from `z` flows into `y` here

error: aborting due to previous error
