
error[E0308]: mismatched types
  --> test.rs:31:36
   |
31 |         self.children[index.index].borrow_mut()
   |                                    ^^^^^^^^^^ lifetime mismatch
   |
   = note: expected type `std::borrow::BorrowMut<NodeChild>`
              found type `std::borrow::BorrowMut<NodeChild + 'a>`
note: the anonymous lifetime #1 defined on the method body at 30:5...
  --> test.rs:30:5
   |
30 | /     pub fn child_at_index_mut(&mut self, index: &NodeChildIndex) -> &mut NodeChild {
31 | |         self.children[index.index].borrow_mut()
32 | |     }
   | |_____^
note: ...does not necessarily outlive the lifetime 'a as defined on the impl at 20:1
  --> test.rs:20:1
   |
20 | / impl<'a> Node<'a> {
21 | |     pub fn new() -> Self {
22 | |         Node { children: Vec::new() }
23 | |     }
...  |
32 | |     }
33 | | }
   | |_^

error: aborting due to previous error(s)
