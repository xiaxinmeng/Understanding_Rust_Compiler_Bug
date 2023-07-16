
../src/librustc/middle/expr_use_visitor.rs:283:9: 287:10 error: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
../src/librustc/middle/expr_use_visitor.rs:283         ExprUseVisitor {
../src/librustc/middle/expr_use_visitor.rs:284             typer: typer,
../src/librustc/middle/expr_use_visitor.rs:285             mc: mc::MemCategorizationContext::new(typer),
../src/librustc/middle/expr_use_visitor.rs:286             delegate: delegate,
../src/librustc/middle/expr_use_visitor.rs:287         }
../src/librustc/middle/expr_use_visitor.rs:279:5: 288:6 help: consider using an explicit lifetime parameter as shown: fn new(delegate: &'d mut Delegate<'tcx>, typer: &'t infer::InferCtxt<'a, 'a>)
 -> ExprUseVisitor<'d, 't, 'a, 'a>
../src/librustc/middle/expr_use_visitor.rs:279     pub fn new(delegate: &'d mut Delegate<'tcx>,
../src/librustc/middle/expr_use_visitor.rs:280                typer: &'t infer::InferCtxt<'a, 'tcx>)
../src/librustc/middle/expr_use_visitor.rs:281                -> ExprUseVisitor<'d,'t,'a,'tcx>
../src/librustc/middle/expr_use_visitor.rs:282     {
../src/librustc/middle/expr_use_visitor.rs:283         ExprUseVisitor {
../src/librustc/middle/expr_use_visitor.rs:284             typer: typer,
                                               ...
error: aborting due to previous error
