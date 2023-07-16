
rust1: internal compiler error: in visit, at rust/typecheck/rust-hir-type-check-path.cc:144
0x1302d7a Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::PathInExpression&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-path.cc:144
0x119b5b1 Rust::HIR::PathInExpression::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:3785
0x127ad67 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:49
0x119bf15 Rust::HIR::CallExpr::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4041
0x127ad67 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:49
0x119b82b Rust::HIR::BorrowExpr::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:3849
0x127ad67 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:49
0x12b2be4 Rust::TyTy::TypeCheckMethodCallExpr::visit(Rust::TyTy::FnType&)
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:3073
0x12aa55f Rust::TyTy::FnType::accept_vis(Rust::TyTy::TyVisitor&)
        ../../gccrs/gcc/rust/typecheck/rust-tyty.cc:1133
0x1279a5a Rust::TyTy::TypeCheckMethodCallExpr::go(Rust::TyTy::BaseType*, Rust::HIR::MethodCallExpr&, Rust::TyTy::BaseType*, Rust::Resolver::TypeCheckContext*)
        ../../gccrs/gcc/rust/typecheck/rust-tyty-call.h:96
0x119bf4d Rust::HIR::MethodCallExpr::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4047
0x127ad67 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:49
0x1272aab Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::BlockExpr&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check.cc:130
0x119bff5 Rust::HIR::BlockExpr::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4065
0x127ad67 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gccrs/gcc/rust/typecheck/rust-hir-type-check-expr.h:49
0x12e18df Rust::Resolver::TraitItemReference::resolve_item(Rust::HIR::TraitItemFunc&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-trait-resolve.cc:122
0x12e1678 Rust::Resolver::TraitItemReference::on_resolved()
        ../../gccrs/gcc/rust/typecheck/rust-hir-trait-resolve.cc:80
0x12775e3 Rust::Resolver::TraitReference::on_resolved()
        ../../gccrs/gcc/rust/typecheck/rust-hir-trait-ref.h:368
0x1279f5b Rust::Resolver::TraitResolver::Resolve(Rust::HIR::TypePath&)
        ../../gccrs/gcc/rust/typecheck/rust-hir-trait-resolve.h:73
0x119cbdb Rust::HIR::ImplBlock::accept_vis(Rust::HIR::HIRFullVisitor&)
        ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4389

