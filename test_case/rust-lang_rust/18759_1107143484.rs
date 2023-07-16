
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime defined here...
 --> src/main.rs:5:13
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |             ^^^^^^^^^
note: ...so that the method type is compatible with trait
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: expected `fn(&mut Mine<'a>) -> Option<&str>`
             found `fn(&mut Mine<'a>) -> Option<&str>`
note: but, the lifetime must be valid for the lifetime `'a` as defined here...
 --> src/main.rs:3:7
  |
3 | impl <'a>Iterator for Mine<'a> {
  |       ^^
note: ...so that the types are compatible
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: expected `<Mine<'a> as Iterator>`
             found `<Mine<'_> as Iterator>`
