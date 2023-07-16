
rust1: internal compiler error: Segmentation fault
0x1053d3f crash_signal
	../../gcc/toplev.cc:322
0x9a0c5b Rust::HIR::Type::clone_type() const
	../../gcc/rust/hir/tree/rust-hir.h:458
0x9a0c5b Rust::HIR::Function::Function(Rust::HIR::Function const&)
	../../gcc/rust/hir/tree/rust-hir-item.h:1117
0x9a0d60 Rust::HIR::Function::clone_item_impl() const
	../../gcc/rust/hir/tree/rust-hir-item.h:1205
0x9aa0e8 Rust::HIR::Item::clone_item() const
	../../gcc/rust/hir/tree/rust-hir.h:175
0x9aa0e8 Crate
	../../gcc/rust/hir/tree/rust-hir.h:891
0x9aa0e8 pair
	/usr/include/c++/4.8/bits/stl_pair.h:127
0x9aa0e8 _Rb_tree_node<const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/stl_tree.h:140
0x9aa0e8 construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/ext/new_allocator.h:120
0x9aa0e8 _S_construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/alloc_traits.h:254
0x9aa0e8 construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/alloc_traits.h:393
0x9aa0e8 _M_create_node<const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/stl_tree.h:406
0x9aa0e8 std::_Rb_tree_iterator<std::pair<unsigned int const, Rust::HIR::Crate> > std::_Rb_tree<unsigned int, std::pair<unsigned int const, Rust::HIR::Crate>, std::_Select1st<std::pair<unsigned int const, Rust::HIR::Crate> >, std::less<unsigned int>, std::allocator<std::pair<unsigned int const, Rust::HIR::Crate> > >::_M_insert_<std::pair<unsigned int const, Rust::HIR::Crate> const&>(std::_Rb_tree_node_base*, std::_Rb_tree_node_base*, std::pair<unsigned int const, Rust::HIR::Crate> const&)
	/usr/include/c++/4.8/bits/stl_tree.h:1023
0x9aa23f std::pair<std::_Rb_tree_iterator<std::pair<unsigned int const, Rust::HIR::Crate> >, bool> std::_Rb_tree<unsigned int, std::pair<unsigned int const, Rust::HIR::Crate>, std::_Select1st<std::pair<unsigned int const, Rust::HIR::Crate> >, std::less<unsigned int>, std::allocator<std::pair<unsigned int const, Rust::HIR::Crate> > >::_M_insert_unique<std::pair<unsigned int const, Rust::HIR::Crate> const&>(std::pair<unsigned int const, Rust::HIR::Crate> const&)
	/usr/include/c++/4.8/bits/stl_tree.h:1382
0x9a52ed std::map<unsigned int, Rust::HIR::Crate, std::less<unsigned int>, std::allocator<std::pair<unsigned int const, Rust::HIR::Crate> > >::insert(std::pair<unsigned int const, Rust::HIR::Crate> const&)
	/usr/include/c++/4.8/bits/stl_map.h:595
0x9a52ed Rust::Analysis::Mappings::insert_hir_crate(Rust::HIR::Crate&&)
	../../gcc/rust/util/rust-hir-map.cc:236
0x91c4f9 Rust::Session::parse_file(char const*)
	../../gcc/rust/rust-session-manager.cc:761
0x91c7c8 Rust::Session::parse_files(int, char const**)
	../../gcc/rust/rust-session-manager.cc:592
