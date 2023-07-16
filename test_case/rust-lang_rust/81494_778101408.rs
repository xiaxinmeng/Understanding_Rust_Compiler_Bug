plain
[RUSTC-TIMING] adler test:false 0.185
[RUSTC-TIMING] panic_abort test:false 0.072
[RUSTC-TIMING] libc test:false 0.720
[RUSTC-TIMING] unwind test:false 0.095
error[E0658]: raw address of syntax is experimental
  --> library/alloc/src/collections/btree/node.rs:76:14
   |
76 |             (&raw mut (*this).parent).write(None);
   |
   = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
   = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
   = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable

error[E0658]: raw address of syntax is experimental
  --> library/alloc/src/collections/btree/node.rs:77:14
   |
77 |             (&raw mut (*this).len).write(0);
   |
   = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
   = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
   = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable

error[E0658]: raw address of syntax is experimental
   --> library/alloc/src/collections/btree/node.rs:120:28
    |
120 |             LeafNode::init(&raw mut (*node.as_mut_ptr()).data);
    |
    = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
    = note: see issue #64490 <https://github.com/rust-lang/rust/issues/64490> for more information
    = help: add `#![feature(raw_ref_op)]` to the crate attributes to enable
[RUSTC-TIMING] compiler_builtins test:false 0.912
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0658`.
