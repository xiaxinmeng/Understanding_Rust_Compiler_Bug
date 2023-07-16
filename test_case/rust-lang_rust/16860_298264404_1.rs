
error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements
 --> test.rs:6:14
  |
6 |         self.a();
  |              ^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the body at 4:20...
 --> test.rs:4:21
  |
4 |       fn b(&mut self) {
  |  _____________________^
5 | |         // Fails
6 | |         self.a();
7 | |
8 | |         // Works
9 | |         (*self).a();
10| |     }
  | |_____^
note: ...so that types are compatible (expected &&mut Self, found &&mut Self)
 --> test.rs:6:14
  |
6 |         self.a();
  |              ^
  = note: but, the lifetime must be valid for the static lifetime...
note: ...so that the type `&mut Self` will meet its required lifetime bounds
 --> test.rs:6:14
  |
6 |         self.a();
  |              ^

error: aborting due to previous error
