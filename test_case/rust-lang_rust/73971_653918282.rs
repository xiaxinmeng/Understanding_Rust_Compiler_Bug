
note: tracking was triggered
   --> /home/r/src/rust/rustc/src/liballoc/collections/btree/node.rs:276:18
    |
276 |         unsafe { &*(self.node.as_ptr() as *mut InternalNode<K, V>) }
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ popped tracked tag for item [Unique for <50139> (call 16054)]
    |
    = note: inside `alloc::collections::btree::node::NodeRef::<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::Internal>::as_internal` at /home/r/src/rust/rustc/src/liballoc/collections/btree/node.rs:276:18
    = note: inside `alloc::collections::btree::node::Handle::<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::Internal>, alloc::collections::btree::node::marker::Edge>::descend` at /home/r/src/rust/rustc/src/liballoc/collections/btree/node.rs:978:20
    = note: inside `alloc::collections::btree::navigate::<impl alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::LeafOrInternal>, alloc::collections::btree::node::marker::KV>>::next_leaf_edge` at /home/r/src/rust/rustc/src/liballoc/collections/btree/navigate.rs:242:17
    = note: inside closure at /home/r/src/rust/rustc/src/liballoc/collections/btree/navigate.rs:134:18
    = note: inside `alloc::collections::btree::navigate::replace::<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>, alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::LeafOrInternal>, alloc::collections::btree::node::marker::KV>, [closure@alloc::collections::btree::navigate::<impl alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut<'a>, K, V, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>::next_unchecked::{{closure}}#0]>` at /home/r/src/rust/rustc/src/liballoc/collections/btree/navigate.rs:88:28
    = note: inside `alloc::collections::btree::navigate::<impl alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, i32, i32, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>::next_unchecked` at /home/r/src/rust/rustc/src/liballoc/collections/btree/navigate.rs:131:22
    = note: inside `std::collections::btree_map::RangeMut::<i32, i32>::next_unchecked` at /home/r/src/rust/rustc/src/liballoc/collections/btree/map.rs:1904:18
    = note: inside `<std::collections::btree_map::IterMut<i32, i32> as std::iter::Iterator>::next` at /home/r/src/rust/rustc/src/liballoc/collections/btree/map.rs:1457:35
    = note: inside `<std::collections::btree_map::ValuesMut<i32, i32> as std::iter::Iterator>::next` at /home/r/src/rust/rustc/src/liballoc/collections/btree/map.rs:1810:9
    = note: inside `std::vec::Vec::<&mut i32>::extend_desugared::<std::collections::btree_map::ValuesMut<i32, i32>>` at /home/r/src/rust/rustc/src/liballoc/vec.rs:2217:35
    = note: inside `<std::vec::Vec<&mut i32> as std::vec::SpecExtend<&mut i32, std::collections::btree_map::ValuesMut<i32, i32>>>::spec_extend` at /home/r/src/rust/rustc/src/liballoc/vec.rs:2110:9
    = note: inside `<std::vec::Vec<&mut i32> as std::vec::SpecExtend<&mut i32, std::collections::btree_map::ValuesMut<i32, i32>>>::from_iter` at /home/r/src/rust/rustc/src/liballoc/vec.rs:2105:9
    = note: inside `<std::vec::Vec<&mut i32> as std::iter::FromIterator<&mut i32>>::from_iter::<std::collections::btree_map::ValuesMut<i32, i32>>` at /home/r/src/rust/rustc/src/liballoc/vec.rs:1995:9
    = note: inside `<std::collections::btree_map::ValuesMut<i32, i32> as std::iter::Iterator>::collect::<std::vec::Vec<&mut i32>>` at /home/r/src/rust/rustc/src/libcore/iter/traits/iterator.rs:1671:9
note: inside `test_all_refs::<i32, std::collections::btree_map::ValuesMut<i32, i32>>` at btree.rs:8:33
   --> btree.rs:8:33
    |
8   |     let mut refs: Vec<&mut T> = iter.collect();
    |                                 ^^^^^^^^^^^^^^
