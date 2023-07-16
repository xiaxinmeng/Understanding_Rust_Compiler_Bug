
$ rustc +devel file.rs -Zborrowck=ast
error[E0495]: cannot infer an appropriate lifetime for autoref due to conflicting requirements
  --> file.rs:12:25
   |
12 |         Some(self.first.as_str())
   |                         ^^^^^^
   |
note: first, the lifetime cannot outlive the lifetime 'b as defined on the method body at 11:13...
  --> file.rs:11:13
   |
11 |     fn next<'b>(&'b mut self) -> Option<Self::Item> {
   |             ^^
note: ...so that reference does not outlive borrowed content
  --> file.rs:12:14
   |
12 |         Some(self.first.as_str())
   |              ^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'a as defined on the impl at 8:6...
  --> file.rs:8:6
   |
8  | impl<'a> Iterator for AnIterator<'a> {
   |      ^^
   = note: ...so that the types are compatible:
           expected std::iter::Iterator
              found std::iter::Iterator

error: aborting due to previous error

For more information about this error, try `rustc --explain E0495`.
