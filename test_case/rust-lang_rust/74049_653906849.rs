
warning: file found to be present in multiple build targets: /tmp/tmp.SnVAq2F49X/fdt-rs/tests/parsing_test.rs
   Compiling fdt-rs v0.2.0 (/tmp/tmp.SnVAq2F49X/fdt-rs)
error: cannot find macro `def_common_iter_funcs` in this scope
  --> src/base/iters.rs:75:1
   |
75 | def_common_iter_funcs!($ DevTreeNode<'a, 'dt>, DevTreeProp<'a, 'dt>, DevTreeNodeIter, DevTreePropIter, DevTreeItem);
   | ^^^^^^^^^^^^^^^^^^^^^

error: cannot find macro `fn_next_node` in this scope
   --> src/base/iters.rs:113:5
    |
113 |     fn_next_node!(
    |     ^^^^^^^^^^^^

error: cannot find macro `fn_next_node_prop` in this scope
   --> src/base/iters.rs:127:5
    |
127 |     fn_next_node_prop!(
    |     ^^^^^^^^^^^^^^^^^

error: cannot find macro `fn_find_next_compatible_node` in this scope
   --> src/base/iters.rs:132:5
    |
132 |     fn_find_next_compatible_node!(
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: cannot find macro `def_common_iter_funcs` in this scope
   --> src/index/iters.rs:111:1
    |
111 | def_common_iter_funcs!($ DevTreeIndexNode<'a, 'i, 'dt>, DevTreeIndexProp<'a, 'i, 'dt>, DevTreeIndexNodeIter, DevTreeIndexPropIter, DevTre...
    | ^^^^^^^^^^^^^^^^^^^^^

error: cannot find macro `fn_next_node` in this scope
   --> src/index/iters.rs:121:5
    |
121 |     fn_next_node!(
    |     ^^^^^^^^^^^^

error: cannot find macro `fn_next_prop` in this scope
   --> src/index/iters.rs:125:5
    |
125 |     fn_next_prop!(
    |     ^^^^^^^^^^^^

error: cannot find macro `fn_next_node_prop` in this scope
   --> src/index/iters.rs:130:5
    |
130 |     fn_next_node_prop!(
    |     ^^^^^^^^^^^^^^^^^

error: cannot find macro `fn_find_next_compatible_node` in this scope
   --> src/index/iters.rs:134:5
    |
134 |     fn_find_next_compatible_node!(
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no method named `find_next_compatible_node` found for struct `base::iters::DevTreeIter<'a, 'dt>` in the current scope
  --> src/base/node.rs:53:25
   |
53 |         self.parse_iter.find_next_compatible_node(string)
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `base::iters::DevTreeIter<'a, 'dt>`
   |
  ::: src/base/iters.rs:61:1
   |
61 | pub struct DevTreeIter<'a, 'dt:'a> {
   | ---------------------------------- method `find_next_compatible_node` not found for this

error[E0599]: no method named `next_node` found for struct `base::iters::DevTreeIter<'_, '_>` in the current scope
  --> src/base/prop.rs:36:34
   |
36 |         self.parent_iter.clone().next_node().unwrap()
   |                                  ^^^^^^^^^ method not found in `base::iters::DevTreeIter<'_, '_>`
   |
  ::: src/base/iters.rs:61:1
   |
61 | pub struct DevTreeIter<'a, 'dt:'a> {
   | ---------------------------------- method `next_node` not found for this

error[E0599]: no method named `find_next_compatible_node` found for struct `base::iters::DevTreeIter<'_, '_>` in the current scope
   --> src/base/tree.rs:211:22
    |
211 |         self.items().find_next_compatible_node(string)
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `base::iters::DevTreeIter<'_, '_>`
    |
   ::: src/base/iters.rs:61:1
    |
61  | pub struct DevTreeIter<'a, 'dt:'a> {
    | ---------------------------------- method `find_next_compatible_node` not found for this

error[E0308]: mismatched types
  --> src/base/iter_macro.rs:22:18
   |
22 |     fn next_prop(&mut iter: Self::TreeItem) -> Option<Self::TreeProp> {
   |                  ^^^^^^^^^-----------------
   |                  |          |
   |                  |          expected due to this
   |                  expected associated type, found `&mut _`
   |                  help: did you mean `iter`: `&<Self as base::iter_macro::ItemIterator>::TreeItem`
   |
   = note: expected associated type `<Self as base::iter_macro::ItemIterator>::TreeItem`
            found mutable reference `&mut _`
   = help: consider constraining the associated type `<Self as base::iter_macro::ItemIterator>::TreeItem` to `&mut _` or calling a method that returns `<Self as base::iter_macro::ItemIterator>::TreeItem`
   = note: for more information, visit https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

error[E0599]: no method named `next_node` found for struct `base::iters::DevTreeIter<'a, 'dt>` in the current scope
   --> src/base/iters.rs:191:16
    |
61  | pub struct DevTreeIter<'a, 'dt:'a> {
    | ---------------------------------- method `next_node` not found for this
...
191 |         self.0.next_node()
    |                ^^^^^^^^^ method not found in `base::iters::DevTreeIter<'a, 'dt>`

error[E0599]: no method named `next_node_prop` found for struct `base::iters::DevTreeIter<'a, 'dt>` in the current scope
   --> src/base/iters.rs:231:16
    |
61  | pub struct DevTreeIter<'a, 'dt:'a> {
    | ---------------------------------- method `next_node_prop` not found for this
...
231 |         self.0.next_node_prop()
    |                ^^^^^^^^^^^^^^ method not found in `base::iters::DevTreeIter<'a, 'dt>`

error[E0599]: no method named `find_next_compatible_node` found for struct `index::iters::DevTreeIndexIter<'_, '_, '_>` in the current scope
   --> src/index/tree.rs:347:22
    |
347 |         self.items().find_next_compatible_node(string)
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `index::iters::DevTreeIndexIter<'_, '_, '_>`
    |
   ::: src/index/iters.rs:104:1
    |
104 | pub struct DevTreeIndexIter<'a, 'i: 'a, 'dt: 'i> {
    | ------------------------------------------------ method `find_next_compatible_node` not found for this

error[E0599]: no method named `next_node` found for struct `index::iters::DevTreeIndexIter<'a, 'i, 'dt>` in the current scope
   --> src/index/iters.rs:26:16
    |
26  |         self.0.next_node()
    |                ^^^^^^^^^ method not found in `index::iters::DevTreeIndexIter<'a, 'i, 'dt>`
...
104 | pub struct DevTreeIndexIter<'a, 'i: 'a, 'dt: 'i> {
    | ------------------------------------------------ method `next_node` not found for this

error[E0599]: no method named `next_node_prop` found for struct `index::iters::DevTreeIndexIter<'a, 'i, 'dt>` in the current scope
   --> src/index/iters.rs:72:16
    |
72  |         self.0.next_node_prop()
    |                ^^^^^^^^^^^^^^ method not found in `index::iters::DevTreeIndexIter<'a, 'i, 'dt>`
...
104 | pub struct DevTreeIndexIter<'a, 'i: 'a, 'dt: 'i> {
    | ------------------------------------------------ method `next_node_prop` not found for this

error[E0599]: no method named `next_prop` found for struct `index::iters::DevTreeIndexIter<'a, 'i, 'dt>` in the current scope
   --> src/index/iters.rs:95:16
    |
95  |         self.0.next_prop()
    |                ^^^^^^^^^ method not found in `index::iters::DevTreeIndexIter<'a, 'i, 'dt>`
...
104 | pub struct DevTreeIndexIter<'a, 'i: 'a, 'dt: 'i> {
    | ------------------------------------------------ method `next_prop` not found for this
    |
    = help: items from traits can only be used if the trait is implemented and in scope
note: `base::iter_macro::ItemIterator` defines an item `next_prop`, perhaps you need to implement it
   --> src/base/iter_macro.rs:17:1
    |
17  | pub trait ItemIterator {
    | ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `crate::prelude`
 --> src/index/iters.rs:1:5
  |
1 | use crate::prelude::*;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error: aborting due to 19 previous errors; 1 warning emitted

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `fdt-rs`.

To learn more, run the command again with --verbose.
