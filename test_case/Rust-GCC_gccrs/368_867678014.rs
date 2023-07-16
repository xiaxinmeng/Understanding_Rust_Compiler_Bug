
rust1: internal compiler error: in TyVar, at rust/typecheck/rust-tyty.cc:41
0x10a3434 Rust::TyTy::TyVar::TyVar(unsigned int)
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:41
0x10a9c3b Rust::TyTy::ParamType::resolve() const
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:1391
0x10aed93 Rust::TyTy::BaseRules::unify(Rust::TyTy::BaseType*)
        ../../gccrs/gcc/rust/typecheck/rust-tyty-rules.h:68
0x10a532b Rust::TyTy::ADTType::unify(Rust::TyTy::BaseType*)
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:420
0xfd7631 Rust::HIR::LetStmt::accept_vis(Rust::HIR::HIRVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4660
0x107b76f operator()
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check.cc:97
0x107d333 _M_invoke
        /usr/include/c++/9/bits/std_function.h:285
0xeec265 Rust::HIR::BlockExpr::iterate_stmts(std::function<bool (Rust::HIR::Stmt*)>)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-expr.h:2484
0x107b97a Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::BlockExpr&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check.cc:96
0xfd64cb Rust::HIR::BlockExpr::accept_vis(Rust::HIR::HIRVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4174
0x1085b47 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:45
0xfd6d1b Rust::HIR::Function::accept_vis(Rust::HIR::HIRVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4414
0x107b67f Rust::Resolver::TypeResolution::Resolve(Rust::HIR::Crate&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check.cc:48
0xedcd70 Rust::Session::parse_file(char const*)
        ../../gccrs/gcc/rust/rust-session-manager.cc:569
0xedc8fe Rust::Session::parse_files(int, char const**)
        ../../gccrs/gcc/rust/rust-session-manager.cc:459
0xe39264 grs_langhook_parse_file
        ../../gccrs/gcc/rust/rust-lang.cc:171

