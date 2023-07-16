
error: cannot infer an appropriate lifetime
 --> src/main.rs:7:16
  |
6 |     fn bar1(&self) -> impl Iterator<Item=u8> {
  |                       ---------------------- this return type evaluates to the `'static` lifetime...
7 |         self.v.iter().cloned()
  |         ------ ^^^^
  |         |
  |         ...but this borrow...
  |
note: ...can't outlive the anonymous lifetime #1 defined on the method body at 6:5
 --> src/main.rs:6:5
  |
6 | /     fn bar1(&self) -> impl Iterator<Item=u8> {
7 | |         self.v.iter().cloned()
8 | |     }
  | |_____^
help: you can add a constraint to the return type to make it last less than `'static` and match the anonymous lifetime #1 defined on the method body at 6:5
  |
6 |     fn bar1(&self) -> impl Iterator<Item=u8> + '_ {
  |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^
