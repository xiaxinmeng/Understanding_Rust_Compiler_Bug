
rust1: internal compiler error: Segmentation fault
0x1b0371b crash_signal
        ../../gccrs/gcc/toplev.cc:322
0x1112a44 Rust::Compile::Context::peek_fn()
        ../../gccrs/gcc/rust/backend/rust-compile-context.h:257
0x1339e9b Rust::Compile::CompileExpr::visit(Rust::HIR::CallExpr&)
        ../../gccrs/gcc/rust/backend/rust-compile-expr.cc:500
0x11b184f Rust::HIR::CallExpr::accept_vis(Rust::HIR::HIRExpressionVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4940
0x111446f Rust::Compile::CompileExpr::Compile(Rust::HIR::Expr*, Rust::Compile::Context*)
        ../../gccrs/gcc/rust/backend/rust-compile-expr.h:38
0x1346794 Rust::Compile::TyTyResolveCompile::visit(Rust::TyTy::ArrayType const&)
        ../../gccrs/gcc/rust/backend/rust-compile-type.cc:342
0x12ceee9 Rust::TyTy::ArrayType::accept_vis(Rust::TyTy::TyConstVisitor&) const
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:1558
0x1112aa9 Rust::Compile::TyTyResolveCompile::compile(Rust::Compile::Context*, Rust::TyTy::BaseType const*, bool)
        ../../gccrs/gcc/rust/backend/rust-compile-type.h:34
0x13454ba Rust::Compile::TyTyResolveCompile::visit(Rust::TyTy::FnType const&)
        ../../gccrs/gcc/rust/backend/rust-compile-type.cc:115
0x12ccddf Rust::TyTy::FnType::accept_vis(Rust::TyTy::TyConstVisitor&) const
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:1167
0x1112aa9 Rust::Compile::TyTyResolveCompile::compile(Rust::Compile::Context*, Rust::TyTy::BaseType const*, bool)
        ../../gccrs/gcc/rust/backend/rust-compile-type.h:34
0x134b3ad Rust::Compile::HIRCompileBase::compile_function(Rust::Compile::Context*, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&, Rust::HIR::SelfParam&, std::vector<Rust::HIR::FunctionParam, std::allocator<Rust::HIR::FunctionParam> >&, Rust::HIR::FunctionQualifiers const&, Rust::HIR::Visibility&, std::vector<Rust::AST::Attribute, std::allocator<Rust::AST::Attribute> >&, Location, Rust::HIR::BlockExpr*, Rust::Resolver::CanonicalPath const*, Rust::TyTy::FnType*, bool)
        ../../gccrs/gcc/rust/backend/rust-compile-base.cc:419
0x1335dfb Rust::Compile::CompileItem::visit(Rust::HIR::Function&)
        ../../gccrs/gcc/rust/backend/rust-compile-item.cc:167
0x11b1fc1 Rust::HIR::Function::accept_vis(Rust::HIR::HIRStmtVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:5144
0x111457b Rust::Compile::CompileInherentImplItem::Compile(Rust::HIR::ImplItem*, Rust::Compile::Context*, Rust::TyTy::BaseType*, bool, Location)
        ../../gccrs/gcc/rust/backend/rust-compile-implitem.h:39
0x1335fa9 Rust::Compile::CompileItem::visit(Rust::HIR::ImplBlock&)
        ../../gccrs/gcc/rust/backend/rust-compile-item.cc:189
0x11b2293 Rust::HIR::ImplBlock::accept_vis(Rust::HIR::HIRStmtVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:5222
0x1112c16 Rust::Compile::CompileItem::compile(Rust::HIR::Item*, Rust::Compile::Context*, Rust::TyTy::BaseType*, bool, Location)
        ../../gccrs/gcc/rust/backend/rust-compile-item.h:37
0x110fb6e Rust::Compile::CompileCrate::go()
        ../../gccrs/gcc/rust/backend/rust-compile.cc:50
0x110fa99 Rust::Compile::CompileCrate::Compile(Rust::HIR::Crate&, Rust::Compile::Context*)
        ../../gccrs/gcc/rust/backend/rust-compile.cc:43
Please submit a full bug report,

