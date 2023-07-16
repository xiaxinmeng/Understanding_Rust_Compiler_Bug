
Breakpoint 5, Rust::HIR::ASTLoweringExprWithBlock::translate (expr=0x3993970, terminated=0x7fffffffd0f8)
    at ../../gccrs/gcc/rust/hir/rust-ast-lower-block.h:131
131	    ASTLoweringExprWithBlock resolver;
(gdb) n
132	    expr->accept_vis (resolver);
(gdb) s
Rust::AST::MatchExpr::accept_vis (this=0x3993970, vis=...)
    at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:5330
5330	  vis.visit (*this);
(gdb) 
Rust::HIR::ASTLoweringBase::visit (this=0x7fffffffd000, expr=...)
    at ../../gccrs/gcc/rust/hir/rust-ast-lower-base.h:130
130	  virtual void visit (AST::MatchExpr &expr) {}
(gdb) 
Rust::HIR::ASTLoweringExprWithBlock::translate (expr=0x3993970, terminated=0x7fffffffd0f8)
    at ../../gccrs/gcc/rust/hir/rust-ast-lower-block.h:133
133	    if (resolver.translated != nullptr)
(gdb) 
141	    *terminated = resolver.terminated;
(gdb) p resolver
$1 = {<Rust::HIR::ASTLoweringBase> = {<Rust::AST::ASTVisitor> = {
      _vptr.ASTVisitor = 0x2ae5008 <vtable for Rust::HIR::ASTLoweringExprWithBlock+16>}, 
    mappings = 0x39905a0}, translated = 0x0, terminated = false}
