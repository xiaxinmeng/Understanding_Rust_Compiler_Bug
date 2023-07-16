rust
error: visibility `pub` is not followed by an item
 --> src/lib.rs:1:1
  |
1 | pub existential type T: MotorCtrl;
  | ^^^ the visibility
  |
  = help: you likely meant to define an item, e.g., `pub fn foo() {}`

error: expected item, found `existential`
 --> src/lib.rs:1:5
  |
1 | pub existential type T: MotorCtrl;
  |     ^^^^^^^^^^^ expected item
