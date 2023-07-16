
error[E0495]: cannot infer an appropriate lifetime for pattern due to conflicting requirements
 --> src/main.rs:5:43
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |                                           ^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #2 defined on the body at 5:38...
 --> src/main.rs:5:38
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |                                      ^^^^^^^^^^
note: ...so that reference does not outlive borrowed content
 --> src/main.rs:5:43
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |                                           ^
note: but, the lifetime must be valid for the method call at 5:13...
 --> src/main.rs:5:13
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...so that a type/lifetime parameter is in scope here
 --> src/main.rs:5:13
  |
5 |     let v = counts.iter().max_by_key(|(_, v)| v);
  |      
