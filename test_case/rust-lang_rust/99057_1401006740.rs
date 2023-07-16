patch 
   Compiling foo v0.1.0 (/home/user/projects/foo)
error[E0432]: unresolved import `crate::Lib`
 --> src/foo.rs:2:5
  |
2 | use crate::Lib;
  |     ^^^^^^^^^^ no `Lib` in the root

For more information about this error, try `rustc --explain E0432`.
- error: could not compile `foo` due to previous error
+ error: could not compile `foo` (build-script) due to previous error
