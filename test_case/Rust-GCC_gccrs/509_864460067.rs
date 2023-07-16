
rust1: internal compiler error: Segmentation fault
0xecee9f crash_signal
        ../../gcc/toplev.c:327
0x7fd82096dd5f ???
        ./signal/../sysdeps/unix/sysv/linux/x86_64/sigaction.c:0
0x9c3949 Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::PathInExpression&)
        ../../gcc/rust/typecheck/rust-hir-type-check-expr.h:875
0x9bf5f5 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gcc/rust/typecheck/rust-hir-type-check-expr.h:48
0x9b9845 Rust::Resolver::TypeCheckStructExpr::visit(Rust::HIR::StructExprStructFields&)
        ../../gcc/rust/typecheck/rust-hir-type-check.cc:129
0x9c542a Rust::Resolver::TypeCheckStructExpr::Resolve(Rust::HIR::StructExprStructFields*)
        ../../gcc/rust/typecheck/rust-hir-type-check-struct-field.h:38
0x9c542a Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::StructExprStructFields&)
        ../../gcc/rust/typecheck/rust-hir-type-check-expr.h:833
0x9bf5f5 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gcc/rust/typecheck/rust-hir-type-check-expr.h:48
0x9c2003 Rust::Resolver::TypeCheckStmt::visit(Rust::HIR::LetStmt&)
        ../../gcc/rust/typecheck/rust-hir-type-check-stmt.h:60
0x9b88e2 Rust::Resolver::TypeCheckStmt::Resolve(Rust::HIR::Stmt*, bool)
        ../../gcc/rust/typecheck/rust-hir-type-check-stmt.h:38
0x9b88e2 operator()
        ../../gcc/rust/typecheck/rust-hir-type-check.cc:97
0x9b88e2 __invoke_impl<bool, Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::BlockExpr&)::<lambda(Rust::HIR::Stmt*)>&, Rust::HIR::Stmt*>
        /usr/include/c++/10/bits/invoke.h:60
0x9b88e2 __invoke_r<bool, Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::BlockExpr&)::<lambda(Rust::HIR::Stmt*)>&, Rust::HIR::Stmt*>
        /usr/include/c++/10/bits/invoke.h:141
0x9b88e2 _M_invoke
        /usr/include/c++/10/bits/std_function.h:291
0x9b8a89 std::function<bool (Rust::HIR::Stmt*)>::operator()(Rust::HIR::Stmt*) const
        /usr/include/c++/10/bits/std_function.h:622
0x9b8a89 Rust::HIR::BlockExpr::iterate_stmts(std::function<bool (Rust::HIR::Stmt*)>)
        ../../gcc/rust/hir/tree/rust-hir-expr.h:2484
0x9b8a89 Rust::Resolver::TypeCheckExpr::visit(Rust::HIR::BlockExpr&)
        ../../gcc/rust/typecheck/rust-hir-type-check.cc:96
0x9bf5f5 Rust::Resolver::TypeCheckExpr::Resolve(Rust::HIR::Expr*, bool)
        ../../gcc/rust/typecheck/rust-hir-type-check-expr.h:48
0x9bfae5 Rust::Resolver::TypeCheckItem::visit(Rust::HIR::Function&)
        ../../gcc/rust/typecheck/rust-hir-type-check-item.h:81
0x9b904b Rust::Resolver::TypeCheckItem::Resolve(Rust::HIR::Item*)
        ../../gcc/rust/typecheck/rust-hir-type-check-item.h:40
Please submit a full bug report,
with preprocessed source if appropriate.
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
