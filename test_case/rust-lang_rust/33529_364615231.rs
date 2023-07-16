
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
 --> src/main.rs:8:31
  |
8 |             None => match err.cause() {
  |                               ^^^^^
  |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 5:1...
 --> src/main.rs:5:1
  |
5 | fn caused_of<'a>(mut err: &'a (Error + 'static)) -> Option<&'a io::Error> {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: ...so that reference does not outlive borrowed content
 --> src/main.rs:8:27
  |
8 |             None => match err.cause() {
  |                           ^^^
  = note: but, the lifetime must be valid for the static lifetime...
  = note: ...so that the expression is assignable:
          expected &'a std::error::Error + 'static
             found &std::error::Error
