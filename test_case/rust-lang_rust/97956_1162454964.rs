
error[E0381]: used binding `pkg` isn't initialized in all conditions
 --> f43.rs:8:10
  |
2 |     let pkg;
  |         --- binding declared here but left uninitialized
...
5 |     } else {
  |      ------ `pkg` is uninitialized if this `else` arm is executed
...
8 |     drop(pkg);
  |          ^^^ `pkg` used here but it isn't initialized in all conditions
