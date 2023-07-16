
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter in generic type due to conflicting requirements
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 5:5...
 --> src/main.rs:5:5
  |
5 |     fn next(&mut self) -> Option<&str> { Some("h") }
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...so that the method type is compatible with trait:
          expected fn(&mut Mine<'a>) -> std::option::Option<&str>
             found fn(&mut Mine<'a>) -> std::option::Option<&str>
note: but, the lifetime must be valid for the lifetime 'a as defined on the impl at 3:1...
 --> src/main.rs:3:1
  |
3 | impl <'a>Iterator for Mine<'a> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  = note: ...so that the types are compatible:
          expected std::iter::Iterator
             found std::iter::Iterator
