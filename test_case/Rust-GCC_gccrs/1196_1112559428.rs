
#0  fancy_abort (file=0x2580950 "../../gccrs/gcc/rust/backend/rust-compile-expr.cc", line=207, 
    function=0x258071e "visit") at ../../gccrs/gcc/diagnostic.cc:2018
#1  0x00000000010d11a8 in Rust::Compile::CompileExpr::visit (this=0x7fffffffcdc0, expr=...)
    at ../../gccrs/gcc/rust/backend/rust-compile-expr.cc:207
#2  0x0000000000f6d172 in Rust::HIR::MatchExpr::accept_vis (this=0x32c0c80, vis=...)
    at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4898
#3  0x0000000000ed6ea5 in Rust::Compile::CompileExpr::Compile (expr=0x32c0c80, ctx=0x7fffffffd720)
    at ../../gccrs/gcc/rust/backend/rust-compile-expr.h:38
#4  0x0000000000ed3340 in Rust::Compile::CompileBlock::visit (this=0x7fffffffcf20, expr=...)
    at ../../gccrs/gcc/rust/backend/rust-compile.cc:93
#5  0x0000000000f6d98e in Rust::HIR::BlockExpr::accept_vis (this=0x32c0d00, vis=...)
    at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:5138
#6  0x0000000000ed5f5d in Rust::Compile::CompileBlock::compile (expr=0x32c0d00, ctx=0x7fffffffd720, result=
    0x0) at ../../gccrs/gcc/rust/backend/rust-compile-block.h:34
#7  0x00000000010d9579 in Rust::Compile::CompileExpr::visit (this=0x7fffffffd0d0, expr=...)
    at ../../gccrs/gcc/rust/backend/rust-compile-expr.h:593
#8  0x0000000000f6d242 in Rust::HIR::LoopExpr::accept_vis (this=0x32cb0e0, vis=...)
    at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:4922
#9  0x0000000000ed6ea5 in Rust::Compile::CompileExpr::Compile (expr=0x32cb0e0, ctx=0x7fffffffd720)
    at ../../gccrs/gcc/rust/backend/rust-compile-expr.h:38
#10 0x0000000000ed7272 in Rust::Compile::CompileStmt::visit (this=0x7fffffffd170, stmt=...)
    at ../../gccrs/gcc/rust/backend/rust-compile-stmt.h:41
#11 0x0000000000f6d820 in Rust::HIR::ExprStmtWithBlock::accept_vis (this=0x32cb260, vis=...)
    at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:5096
#12 0x0000000000ed7229 in Rust::Compile::CompileStmt::Compile (stmt=0x32cb260, ctx=0x7fffffffd720)
    at ../../gccrs/gcc/rust/backend/rust-compile-stmt.h:35
#13 0x00000000010e1e65 in Rust::Compile::HIRCompileBase::compile_function_body (ctx=0x7fffffffd720, fndecl=
    <function_decl 0x7ffff7453100 bar::bar>, function_body=..., has_return_type=true)
    at ../../gccrs/gcc/rust/backend/rust-compile-base.cc:374
#14 0x00000000010e278e in Rust::Compile::HIRCompileBase::compile_function (ctx=0x7fffffffd720, fn_name=..., 
    self_param=..., function_params=..., qualifiers=..., visibility=..., outer_attrs=..., locus=..., 
    function_body=0x32c1180, canonical_path=0x32c5af8, fntype=0x3298360, function_has_return=true)
    at ../../gccrs/gcc/rust/backend/rust-compile-base.cc:521
#15 0x00000000010ce87f in Rust::Compile::CompileItem::visit (this=0x7fffffffd610, function=...)
    at ../../gccrs/gcc/rust/backend/rust-compile-item.cc:167
#16 0x0000000000f6d9c0 in Rust::HIR::Function::accept_vis (this=0x32985c0, vis=...)
    at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:5144
#17 0x0000000000ed5c6a in Rust::Compile::CompileItem::compile (item=0x32985c0, ctx=0x7fffffffd720, 
    concrete=0x0, is_query_mode=false, ref_locus=...) at ../../gccrs/gcc/rust/backend/rust-compile-item.h:37
#18 0x0000000000ed306b in Rust::Compile::CompileCrate::go (this=0x7fffffffd6f0)
    at ../../gccrs/gcc/rust/backend/rust-compile.cc:50
#19 0x0000000000ed2fd3 in Rust::Compile::CompileCrate::Compile (crate=..., ctx=0x7fffffffd720)
    at ../../gccrs/gcc/rust/backend/rust-compile.cc:43
#20 0x0000000000ebd974 in Rust::Session::parse_file (
...
//  rust-compile-expr.cc:207: rust_assert (scrutinee_expr_tyty->get_kind () == TyTy::TypeKind::ADT);
(gdb) p scrutinee_expr_tyty->get_kind ()
$1 = Rust::TyTy::BOOL
