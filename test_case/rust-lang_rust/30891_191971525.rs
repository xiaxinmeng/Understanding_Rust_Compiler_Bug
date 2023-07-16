
(gdb) bt
#0  0x00007ff9f17ce507 in __GI_raise (sig=sig@entry=6)
    at ../sysdeps/unix/sysv/linux/raise.c:55
#1  0x00007ff9f17cf8da in __GI_abort () at abort.c:89
#2  0x00007ff9f17c759d in __assert_fail_base (
    fmt=0x7ff9f1904778 "%s%s%s:%u: %s%sAssertion `%s' failed.\n%n", 
    assertion=assertion@entry=0x7ff9eaeed5c8 "C1->getType() == C2->getType() && \"Operand types in binary constant expression should match\"", 
    file=file@entry=0x7ff9eaeec338 "/home/ariel/Rust/rust/src/llvm/lib/IR/Constants.cpp", line=line@entry=1810, 
    function=function@entry=0x7ff9eaef0ac0 <llvm::ConstantExpr::get(unsigned int, llvm::Constant*, llvm::Constant*, unsigned int, llvm::Type*)::__PRETTY_FUNCTION__> "static llvm::Constant* llvm::ConstantExpr::get(unsigned int, llvm::Constant*, llvm::Constant*, unsigned int, llvm::Type*)") at assert.c:92
#3  0x00007ff9f17c7652 in __GI___assert_fail (
    assertion=0x7ff9eaeed5c8 "C1->getType() == C2->getType() && \"Operand types in binary constant expression should match\"", 
    file=0x7ff9eaeec338 "/home/ariel/Rust/rust/src/llvm/lib/IR/Constants.cpp", 
    line=1810, 
    function=0x7ff9eaef0ac0 <llvm::ConstantExpr::get(unsigned int, llvm::Constant*, llvm::Constant*, unsigned int, llvm::Type*)::__PRETTY_FUNCTION__> "static llvm::Constant* llvm::ConstantExpr::get(unsigned int, llvm::Constant*, llvm::Constant*, unsigned int, llvm::Type*)") at assert.c:101
#4  0x00007ff9ea16e8e0 in llvm::ConstantExpr::get(unsigned int, llvm::Constant*, llvm::Constant*, unsigned int, llvm::Type*) ()
   from /home/ariel/Rust/rust/build-llvm-assertions/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_llvm-9026086f.so
#5  0x00007ff9ef729fef in rustc_trans::trans::consts::const_expr_unadjusted (
    cx=0x7ff9e6f68b68, e=0x7ff9e0079490, ety=0x7ff9e006fc30, 
    param_substs=0x7ff9e00bf5b0, fn_args=None, trueconst=No)
    at ../src/librustc_trans/trans/consts.rs:614
#6  0x00007ff9ef722fe7 in rustc_trans::trans::consts::const_expr (
    cx=0x7ff9e6f68b68, e=0x7ff9e0079490, param_substs=0x7ff9e00bf5b0, 
    fn_args=None, trueconst=No) at ../src/librustc_trans/trans/consts.rs:347
#7  0x00007ff9ef7258a2 in rustc_trans::trans::consts::get_const_expr_as_global (
    ccx=0x7ff9e6f68b68, expr=0x7ff9e0079490, qualif=ConstQualif = {...}, 
    param_substs=0x7ff9e00bf5b0, trueconst=No)
    at ../src/librustc_trans/trans/consts.rs:321
#8  0x00007ff9ef54648a in rustc_trans::trans::expr::trans (bcx=0x7ff9e01090f0, 
    expr=0x7ff9e0079490) at ../src/librustc_trans/trans/expr.rs:247
#9  0x00007ff9ef75ca16 in rustc_trans::trans::controlflow::trans_if (
    bcx=0x7ff9e01090f0, if_id=10, cond=0x7ff9e0079490, thn=0x7ff9e0079560, 
    els=None, dest=Ignore) at ../src/librustc_trans/trans/macros.rs:166
#10 0x00007ff9ef7c769a in rustc_trans::trans::expr::trans_rvalue_dps_unadjusted (
    bcx=0x7ff9e01090f0, expr=0x7ff9e007c2a0, dest=Ignore)
    at ../src/librustc_trans/trans/expr.rs:1074
#11 0x00007ff9ef5fc0e2 in rustc_trans::trans::expr::trans_into (
    bcx=0x7ff9e01090f0, expr=0x7ff9e007c2a0, dest=Ignore)
    at ../src/librustc_trans/trans/expr.rs:224
#12 0x00007ff9ef5f3e17 in rustc_trans::trans::controlflow::trans_block (
    bcx=0x7ff9e01090f0, b=0x7ff9e00793f0, dest=Ignore)
    at ../src/librustc_trans/trans/controlflow.rs:135
#13 0x00007ff9ef5efb9b in rustc_trans::trans::base::trans_closure (
    ccx=0x7ff9e6f68b68, decl=0x7ff9e004cfc0, body=0x7ff9e00793f0, 
    llfndecl=0x7ff9e0108478, param_substs=0x7ff9e00bf5b0, fn_ast_id=7, 
    attributes=&[syntax::codemap::Spanned<syntax::ast::Attribute_>](len: 0), 
    output_type=FnConverging = {...}, abi=Rust, closure_env=NotClosure)
    at ../src/librustc_trans/trans/base.rs:2067
#14 0x00007ff9ef5f62d6 in rustc_trans::trans::base::trans_fn (ccx=0x7ff9e6f68b68, 
    decl=0x7ff9e004cfc0, body=0x7ff9e00793f0, llfndecl=0x7ff9e0108478, 
    param_substs=0x7ff9e00bf5b0, id=7, 
    attrs=&[syntax::codemap::Spanned<syntax::ast::Attribute_>](len: 0))
    at ../src/librustc_trans/trans/base.rs:2143
#15 0x00007ff9ef602a08 in rustc_trans::trans::base::trans_item (
    ccx=0x7ff9e6f6a018, item=0x7ff9e007c870)
    at ../src/librustc_trans/trans/base.rs:2520
#16 0x00007ff9ef6645ba in fnfn () at ../src/librustc_trans/trans/base.rs:3379
#17 0x00007ff9ef6644c9 in rustc_trans::dep_graph::DepGraph::with_task<closure,()> (
    self=0x7ff9e6f6d8d8, key=TransCrateItem = {...}, op=closure = {...})
    at ../src/librustc/dep_graph/mod.rs:162
#18 0x00007ff9ef664441 in rustc_trans::trans::base::TransItemsWithinModVisitor<'a, 'tcx>.Visitor<'v>::visit_item (self=0x7ff9e6f6a000, i=0x7ff9e007c870)
    at ../src/librustc_trans/trans/base.rs:3369
