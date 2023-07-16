
error: line not found in debugger output: $1 = BTreeSet<i32>(len: 3) = {3, 5, 7}
[...]
$1 = alloc::collections::btree::set::BTreeSet<i32> {map: alloc::collections::btree::map::BTreeMap<i32, ()> {root: alloc::collections::btree::node::Root<i32, ()> {node: alloc::collections::btree::node::BoxedNode<i32, ()> {ptr: core::ptr::Unique<alloc::collections::btree::node::LeafNode<i32, ()>> {pointer: core::nonzero::NonZero<*const alloc::collections::btree::node::LeafNode<i32, ()>> (0x555555774130), _marker: core::marker::PhantomData<alloc::collections::btree::node::LeafNode<i32, ()>>}}, height: 0}, length: 3}}
