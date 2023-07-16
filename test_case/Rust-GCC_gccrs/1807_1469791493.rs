
rust1: internal compiler error: Segmentation fault
0x1173f23 crash_signal
	../../gccrs/gcc/toplev.cc:314
0x7f5a480f851f ???
	./signal/../sysdeps/unix/sysv/linux/x86_64/libc_sigaction.c:0
0xafcee4 Rust::Analysis::NodeMapping::get_hirid() const
	../../gccrs/gcc/rust/util/rust-hir-map.cc:51
0xa521d3 Rust::Compile::HIRCompileBase::query_compile(unsigned int, Rust::TyTy::BaseType*, Rust::HIR::PathIdentSegment const&, Rust::Analysis::NodeMapping const&, Location, bool)
	../../gccrs/gcc/rust/backend/rust-compile-resolve-path.cc:238
0xa52b24 Rust::Compile::ResolvePathRef::resolve(Rust::HIR::PathIdentSegment const&, Rust::Analysis::NodeMapping const&, Location, bool)
	../../gccrs/gcc/rust/backend/rust-compile-resolve-path.cc:168
0xa52ce3 Rust::Compile::ResolvePathRef::visit(Rust::HIR::PathInExpression&)
	../../gccrs/gcc/rust/backend/rust-compile-resolve-path.cc:42
0xbe4e0d ???
	/usr/include/c++/11/bits/basic_string.tcc:233
0xbe5784 Rust::Compile::CompileExpr::Compile(Rust::HIR::Expr*, Rust::Compile::Context*)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:48
0xbe58c4 Rust::Compile::CompileExpr::visit(Rust::HIR::ComparisonExpr&)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:250
0xbe5784 Rust::Compile::CompileExpr::Compile(Rust::HIR::Expr*, Rust::Compile::Context*)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:48
0xc0c570 Rust::Compile::HIRCompileBase::compile_function_body(Rust::Compile::Context*, tree_node*, Rust::HIR::BlockExpr&, bool)
	../../gccrs/gcc/rust/backend/rust-compile-base.cc:485
0xbedd88 Rust::Compile::CompileExpr::generate_closure_function(Rust::HIR::ClosureExpr&, Rust::TyTy::ClosureType&, tree_node*)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:2984
0xbedf87 Rust::Compile::CompileExpr::visit(Rust::HIR::ClosureExpr&)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:2809
0xbe5784 Rust::Compile::CompileExpr::Compile(Rust::HIR::Expr*, Rust::Compile::Context*)
	../../gccrs/gcc/rust/backend/rust-compile-expr.cc:48
0xbe4416 Rust::Compile::CompileStmt::visit(Rust::HIR::LetStmt&)
	../../gccrs/gcc/rust/backend/rust-compile-stmt.cc:69
0xbe45d1 Rust::Compile::CompileStmt::Compile(Rust::HIR::Stmt*, Rust::Compile::Context*)
	../../gccrs/gcc/rust/backend/rust-compile-stmt.cc:34
0xc0c51a Rust::Compile::HIRCompileBase::compile_function_body(Rust::Compile::Context*, tree_node*, Rust::HIR::BlockExpr&, bool)
	../../gccrs/gcc/rust/backend/rust-compile-base.cc:472
0xc0db37 Rust::Compile::HIRCompileBase::compile_function(Rust::Compile::Context*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, Rust::HIR::SelfParam&, std::vector<Rust::HIR::FunctionParam, std::allocator<Rust::HIR::FunctionParam> >&, Rust::HIR::FunctionQualifiers const&, Rust::HIR::Visibility&, std::vector<Rust::AST::Attribute, std::allocator<Rust::AST::Attribute> >&, Location, Rust::HIR::BlockExpr*, Rust::Resolver::CanonicalPath const*, Rust::TyTy::FnType*, bool)
	../../gccrs/gcc/rust/backend/rust-compile-base.cc:623
0xbe38fe Rust::Compile::CompileItem::visit(Rust::HIR::Function&)
	../../gccrs/gcc/rust/backend/rust-compile-item.cc:174
0xa4e579 Rust::Compile::CompileItem::compile(Rust::HIR::Item*, Rust::Compile::Context*, Rust::TyTy::BaseType*, bool, Location)
	../../gccrs/gcc/rust/backend/rust-compile-item.h:37
Please submit a full bug report, with preprocessed source (by using -freport-bug).
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
  