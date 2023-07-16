
error[E0716]: temporary value dropped while borrowed
   --> library\alloc\src\collections\btree\node.rs:632:46
    |
632 |         let leaf = NodeRef::as_leaf_mut(&mut self.borrow_mut());
    |                                              ^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
    |                                              |
    |                                              creates a temporary which is freed while still in use
633 |         leaf.parent = None;
    |         ------------------ borrow later used here
    |
