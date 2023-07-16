
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
  --> src/main.rs:10:16
   |
10 |         self.x.iter().map(|a| a.0)
   |                ^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 9:5...
  --> src/main.rs:9:5
   |
9  | /     fn iter_values(&self) -> impl Iterator<Item=u32>{
10 | |         self.x.iter().map(|a| a.0)
11 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:10:9
   |
10 |         self.x.iter().map(|a| a.0)
   |         ^^^^^^
   = note: but, the lifetime must be valid for the static lifetime...
note: ...so that return value is valid for the call
  --> src/main.rs:9:30
   |
9  |     fn iter_values(&self) -> impl Iterator<Item=u32>{
   |                              ^^^^^^^^^^^^^^^^^^^^^^^
