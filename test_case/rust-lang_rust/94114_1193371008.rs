plain
    Checking rand v0.7.3
    Checking core v0.0.0 (/checkout/library/core)
    Checking alloc v0.0.0 (/checkout/library/alloc)
    Checking std v0.0.0 (/checkout/library/std)
error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/alloc/src/collections/btree/map/tests.rs:119:30
    |
119 |             self.root.insert(Root::new(*self.alloc)).bulk_push(iter, &mut self.length, *self.alloc);
    |                              ^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
   --> library/alloc/src/collections/btree/map/tests.rs:119:13
    |
119 |             self.root.insert(Root::new(*self.alloc)).bulk_push(iter, &mut self.length, *self.alloc);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
    |
    = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/alloc/src/collections/btree/node/tests.rs:71:21
   |
71 |     let mut root1 = NodeRef::new_leaf(Global);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/alloc/src/collections/btree/node/tests.rs:73:21
   |
73 |     let mut root1 = NodeRef::new_internal(root1.forget_type(), Global).forget_type();
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

error[E0133]: call to unsafe function is unsafe and requires unsafe block
  --> library/alloc/src/collections/btree/node/tests.rs:74:17
   |
74 |     let root2 = Root::new(Global);
   |                 ^^^^^^^^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior
For more information about this error, try `rustc --explain E0133`.
error: could not compile `alloc` due to 5 previous errors
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:01:27
