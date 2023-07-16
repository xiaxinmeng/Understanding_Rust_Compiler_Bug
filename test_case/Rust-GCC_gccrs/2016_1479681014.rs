
./../gcc/rust/expand/rust-expand-visitor.cc: In member function 'virtual void Rust::ExpandVisitor::visit(Rust::AST::TupleExpr&)':
../../gcc/rust/expand/rust-expand-visitor.cc:421:21: error: use of 'expr' before deduction of 'auto'
   for (auto &expr : expr.get_tuple_elems ())
                     ^
../../gcc/rust/Make-lang.in:386: recipe for target 'rust/rust-expand-visitor.o' failed
make[2]: *** [rust/rust-expand-visitor.o] Error 1
make[2]: *** Waiting for unfinished jobs....
In file included from ../../gcc/rust/ast/rust-ast-full.h:22:0,
                 from ../../gcc/rust/parse/rust-parse.h:21,
                 from ../../gcc/rust/expand/rust-macro-expand.h:23,
                 from ../../gcc/rust/expand/rust-macro-expand.cc:19:
../../gcc/rust/ast/rust-ast.h: In member function 'Rust::AST::LifetimeParam Rust::Parser<ManagedTokenSource>::parse_lifetime_param() [with ManagedTokenSource = Rust::MacroInvocLexer]':
../../gcc/rust/ast/rust-ast.h:1210:7: warning: 'lifetime.Rust::AST::Lifetime::node_id' may be used uninitialized in this function [-Wmaybe-uninitialized]
 class Lifetime : public TypeParamBound
       ^
In file included from ../../gcc/rust/parse/rust-parse.h:734:0,
                 from ../../gcc/rust/expand/rust-macro-expand.h:23,
                 from ../../gcc/rust/expand/rust-macro-expand.cc:19:
../../gcc/rust/parse/rust-parse-impl.h:3316:17: note: 'lifetime.Rust::AST::Lifetime::node_id' was declared here
   AST::Lifetime lifetime (AST::Lifetime::NAMED, lifetime_tok->get_str (),
                 ^
