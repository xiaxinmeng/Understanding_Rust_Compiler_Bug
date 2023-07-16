
error[E0495]: cannot infer an appropriate lifetime for borrow expression due to conflicting requirements
  --> src/main.rs:42:14
   |
42 |         Some(&mut self.cont.item)
   |              ^^^^^^^^^^^^^^^^^^^
   |
note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the method body at 39:5...
  --> src/main.rs:39:5
   |
39 | /     fn next(&mut self) -> Option<&'a mut V> {
40 | |         if self.emitted { return None }
41 | |         self.emitted = true;
42 | |         Some(&mut self.cont.item)
43 | |     }
   | |_____^
note: ...so that reference does not outlive borrowed content
  --> src/main.rs:42:14
   |
42 |         Some(&mut self.cont.item)
   |              ^^^^^^^^^^^^^^^^^^^
note: but, the lifetime must be valid for the lifetime 'a as defined on the impl at 37:1...
  --> src/main.rs:37:1
   |
37 | impl<'a, V:'a> Iterator for MutableIter<'a,V> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: ...so that the expression is assignable:
           expected std::option::Option<&'a mut V>
              found std::option::Option<&mut V>
