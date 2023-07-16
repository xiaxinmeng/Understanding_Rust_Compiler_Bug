
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
 --> src/main.rs:8:5
  |
8 |     fn borrow(&self) -> &T {
  |     ^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 8:5...
 --> src/main.rs:8:5
  |
8 | /     fn borrow(&self) -> &T {
9 | |         self as &T
10| |     }
  | |_____^
  = note: ...but the lifetime must also be valid for the static lifetime...
  = note: ...so that the method type is compatible with trait:
          expected fn(&S) -> &T + 'static
             found fn(&S) -> &T
