
rust1: internal compiler error: Segmentation fault
0x1053c9f crash_signal
	../../gcc/toplev.cc:322
0x995ad4 Rust::HIR::Type::clone_type() const
	../../gcc/rust/hir/tree/rust-hir.h:458
0x995ad4 LetStmt
	../../gcc/rust/hir/tree/rust-hir-stmt.h:96
0x995ad4 Rust::HIR::LetStmt::clone_stmt_impl() const
	../../gcc/rust/hir/tree/rust-hir-stmt.h:131
0x99e159 Rust::HIR::Stmt::clone_stmt() const
	../../gcc/rust/hir/tree/rust-hir.h:135
0x99e159 Rust::HIR::BlockExpr::BlockExpr(Rust::HIR::BlockExpr const&)
	../../gcc/rust/hir/tree/rust-hir-expr.h:2137
0x9a0de2 Rust::HIR::BlockExpr::clone_block_expr_impl() const
	../../gcc/rust/hir/tree/rust-hir-expr.h:2207
0x9a0de2 Rust::HIR::BlockExpr::clone_block_expr() const
	../../gcc/rust/hir/tree/rust-hir-expr.h:2165
0x9a0de2 Rust::HIR::Function::Function(Rust::HIR::Function const&)
	../../gcc/rust/hir/tree/rust-hir-item.h:1116
0x9a0f00 Rust::HIR::Function::clone_item_impl() const
	../../gcc/rust/hir/tree/rust-hir-item.h:1216
0x9aa2b8 Rust::HIR::Item::clone_item() const
	../../gcc/rust/hir/tree/rust-hir.h:175
0x9aa2b8 Crate
	../../gcc/rust/hir/tree/rust-hir.h:891
0x9aa2b8 pair
	/usr/include/c++/4.8/bits/stl_pair.h:127
0x9aa2b8 _Rb_tree_node<const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/stl_tree.h:140
0x9aa2b8 construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/ext/new_allocator.h:120
0x9aa2b8 _S_construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/alloc_traits.h:254
0x9aa2b8 construct<std::_Rb_tree_node<std::pair<unsigned int const, Rust::HIR::Crate> >, const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/alloc_traits.h:393
0x9aa2b8 _M_create_node<const std::pair<unsigned int const, Rust::HIR::Crate>&>
	/usr/include/c++/4.8/bits/stl_tree.h:406
0x9aa2b8 std::_Rb_tree_iterator<std::pair<unsigned int const, Rust::HIR::Crate> > std::_Rb_tree<unsigned int, std::pair<unsigned int const, Rust::HIR::Crate>, std::_Select1st<std::pair<unsigned int const, Rust::HIR::Crate> >, std::less<unsigned int>, std::allocator<std::pair<unsigned int const, Rust::HIR::Crate> > >::_M_insert_<std::pair<unsigned int const, Rust::HIR::Crate> const&>(std::_Rb_tree_node_base*, std::_Rb_tree_node_base*, std::pair<unsigned int const, Rust::HIR::Crate> const&)
	/usr/include/c++/4.8/bits/stl_tree.h:1023
0x9aa40f std::pair<std::_Rb_tree_iterator<std::pair<unsigned int const, Rust::HIR::Crate> >, bool> std::_Rb_tree<unsigned int, std::pair<unsigned int const, Rust::HIR::Crate>, std::_Select1st<std::pair<unsigned int const, Rust::HIR::Crate> >, std::less<unsigned int>, std::allocator<std::pair<unsigned int const, Rust::HIR::Crate> > >::_M_insert_unique<std::pair<unsigned int const, Rust::HIR::Crate> const&>(std::pair<unsigned int const, Rust::HIR::Crate> const&)
	/usr/include/c++/4.8/bits/stl_tree.h:1382
