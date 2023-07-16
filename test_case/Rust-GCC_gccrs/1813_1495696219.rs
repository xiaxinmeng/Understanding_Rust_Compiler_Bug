
crab1: internal compiler error: in check_match_scrutinee, at rust/backend/rust-compile-expr.cc:1267
0x2406e9e internal_error(char const*, ...)
	???:0
0xae25b2 fancy_abort(char const*, int, char const*)
	???:0
0xd9a85b Rust::Compile::CompileExpr::visit(Rust::HIR::MatchExpr&)
	???:0
0xd8fcb4 Rust::Compile::CompileExpr::Compile(Rust::HIR::Expr*, Rust::Compile::Context*)
	???:0
0xdb7a80 Rust::Compile::HIRCompileBase::compile_function_body(Rust::Compile::Context*, tree_node*, Rust::HIR::BlockExpr&, bool)
	???:0
0xdb95a0 Rust::Compile::HIRCompileBase::compile_function(Rust::Compile::Context*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, Rust::HIR::SelfParam&, std::vector<Rust::HIR::FunctionParam, std::allocator<Rust::HIR::FunctionParam> >&, Rust::HIR::FunctionQualifiers const&, Rust::HIR::Visibility&, std::vector<Rust::AST::Attribute, std::allocator<Rust::AST::Attribute> >&, Location, Rust::HIR::BlockExpr*, Rust::Resolver::CanonicalPath const*, Rust::TyTy::FnType*, bool)
	???:0
0xd8d8cb Rust::Compile::CompileItem::visit(Rust::HIR::Function&)
	???:0
0xbf64f9 Rust::Compile::CompileCrate::go()
	???:0
0xbf6548 Rust::Compile::CompileCrate::Compile(Rust::HIR::Crate&, Rust::Compile::Context*)
	???:0
0xbee1c9 Rust::Session::compile_crate(char const*)
	???:0
0xbee47b Rust::Session::handle_input_files(int, char const**)
	???:0
