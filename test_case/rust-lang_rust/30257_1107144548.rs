
error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
  --> src/main.rs:42:14
   |
42 |         Some(&mut self.cont.item)
   |              ^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime defined here...
  --> src/main.rs:39:13
   |
39 |     fn next(&mut self) -> Option<&'a mut V> {
   |             ^^^^^^^^^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:42:14
   |
42 |         Some(&mut self.cont.item)
   |              ^^^^^^^^^^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime `'a` as defined here...
  --> src/main.rs:37:6
   |
37 | impl<'a, V:'a> Iterator for MutableIter<'a,V> {
   |      ^^
note: ...so that the types are compatible
  --> src/main.rs:42:9
   |
42 |         Some(&mut self.cont.item)
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected `Option<&'a mut V>`
              found `Option<&mut V>`
