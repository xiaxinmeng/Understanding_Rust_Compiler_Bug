rust
note: the anonymous lifetime #1 defined on the body at 30:83...
  --> file.rs:30:84
   |
30 |       pub fn child_at_index_mut(&mut self, index: &NodeChildIndex) -> &mut NodeChild {
   |  ____________________________________________________________________________________^ starting here...
31 | |         self.children[index.index].borrow_mut()
32 | |     }
   | |_____^ ...ending here
note: ...does not necessarily outlive the lifetime 'a as defined on the body at 30:83
  --> file.rs:30:84
   |
30 |       pub fn child_at_index_mut(&mut self, index: &NodeChildIndex) -> &mut NodeChild {
   |  ____________________________________________________________________________________^ starting here...
31 | |         self.children[index.index].borrow_mut()
32 | |     }
   | |_____^ ...ending here
