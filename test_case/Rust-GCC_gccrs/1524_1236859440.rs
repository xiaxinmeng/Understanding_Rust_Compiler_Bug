
Program received signal SIGSEGV, Segmentation fault.
Rust::Resolver::ResolveRelativeQualTypePath::go (path=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:286
286       associated->accept_vis (o);
(gdb) 
(gdb) bt
#0  Rust::Resolver::ResolveRelativeQualTypePath::go (path=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:286
#1  0x00000000013072d6 in Rust::Resolver::ResolveType::visit (this=0x7fffffffcaf0, path=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:95
#2  0x0000000001086a2a in Rust::AST::QualifiedPathInType::accept_vis (this=0x4877d50, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:4946
#3  0x00000000012fe455 in Rust::Resolver::ResolveType::go (type=0x4877d50) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:63
#4  0x0000000001306bdb in Rust::Resolver::ResolveGenericArgs::resolve_disambiguated_generic (this=0x7fffffffcbb0, arg=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:551
#5  0x0000000001306d1c in Rust::Resolver::ResolveGenericArgs::go (generic_args=..., prefix=..., canonical_prefix=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:577
#6  0x0000000001306c3e in Rust::Resolver::ResolveGenericArgs::go (generic_args=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:562
#7  0x0000000001305251 in Rust::Resolver::ResolveRelativeTypePath::go (path=..., resolved_node_id=@0x7fffffffcd88: 0) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.cc:143
#8  0x00000000013072b3 in Rust::Resolver::ResolveType::visit (this=0x7fffffffcd70, path=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:90
#9  0x00000000010869b4 in Rust::AST::TypePath::accept_vis (this=0x4877ea8, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:4934
#10 0x00000000012fe455 in Rust::Resolver::ResolveType::go (type=0x4877ea8) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:63
#11 0x00000000012fe548 in Rust::Resolver::ResolveTypeBound::visit (this=0x7fffffffce10, bound=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:132
#12 0x00000000010884e6 in Rust::AST::TraitBound::accept_vis (this=0x4877e80, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:5666
#13 0x00000000012fe4f5 in Rust::Resolver::ResolveTypeBound::go (type=0x4877e80) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:126
#14 0x00000000012feaef in Rust::Resolver::ResolveWhereClause::visit (this=0x7fffffffcf00, item=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:221
#15 0x000000000108777c in Rust::AST::TypeBoundWhereClauseItem::accept_vis (this=0x4876af0, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:5312
#16 0x00000000012fea04 in Rust::Resolver::ResolveWhereClause::Resolve (where_clause=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-type.h:211
#17 0x00000000012f8ea8 in Rust::Resolver::ResolveItem::visit (this=0x7fffffffd140, method=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-item.cc:614
#18 0x00000000010877b4 in Rust::AST::Method::accept_vis (this=0x4877ef0, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:5318
#19 0x00000000012fab0d in Rust::Resolver::ResolveImplItems::go (item=0x4877ef0, prefix=..., canonical_prefix=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-item.cc:988
#20 0x00000000012fa38d in Rust::Resolver::ResolveItem::resolve_impl_item (this=0x7fffffffd3a0, item=0x4877ef0, prefix=..., canonical_prefix=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-item.cc:839
#21 0x00000000012f8a38 in Rust::Resolver::ResolveItem::visit (this=0x7fffffffd3a0, impl_block=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-item.cc:578
#22 0x0000000001087d30 in Rust::AST::InherentImpl::accept_vis (this=0x4878170, vis=...) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:5462
#23 0x00000000012f5f6e in Rust::Resolver::ResolveItem::go (item=0x4878170, prefix=..., canonical_prefix=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve-item.cc:188
#24 0x00000000012e8c79 in Rust::Resolver::NameResolution::go (this=0x487f3d0, crate=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve.cc:108
#25 0x00000000012e8892 in Rust::Resolver::NameResolution::Resolve (crate=...) at ../../gccrs/gcc/rust/resolve/rust-ast-resolve.cc:58
#26 0x000000000118081f in Rust::Session::compile_crate (this=0x4646180 <Rust::Session::get_instance()::instance>, filename=0x7fffffffe070 "test.rs") at ../../gccrs/gcc/rust/rust-session-manager.cc:518
#27 0x000000000117fee9 in Rust::Session::handle_input_files (this=0x4646180 <Rust::Session::get_instance()::instance>, num_files=1, files=0x4858d00) at ../../gccrs/gcc/rust/rust-session-manager.cc:363
#28 0x00000000010300e8 in grs_langhook_parse_file () at ../../gccrs/gcc/rust/rust-lang.cc:180
#29 0x0000000001c1097a in compile_file () at ../../gccrs/gcc/toplev.cc:452
#30 0x0000000001c13b1d in do_compile (no_backend=false) at ../../gccrs/gcc/toplev.cc:2146
#31 0x0000000001c13f19 in toplev::main (this=0x7fffffffdaf2, argc=10, argv=0x7fffffffdc28) at ../../gccrs/gcc/toplev.cc:2298
#32 0x0000000003509d4b in main (argc=10, argv=0x7fffffffdc28) at ../../gccrs/gcc/main.cc:39
