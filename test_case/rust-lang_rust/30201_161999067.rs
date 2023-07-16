
../src/librustc/middle/expr_use_visitor.rs:285:9: 285:68 error: cannot infer an appropriate lifetime for lifetime parameter `'tcx` due to conflicting requirements
../src/librustc/middle/expr_use_visitor.rs:285         ExprUseVisitor { typer: typer, mc: mc, delegate: delegate }
                                                       ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
../src/librustc/middle/expr_use_visitor.rs:279:5: 286:6 help: consider using an explicit lifetime parameter as shown: fn new(delegate: &'d mut (Delegate<'tcx>),
       typer: &'t infer::InferCtxt<'a, 'a>) -> ExprUseVisitor<'d, 't, 'a, 'a>
 where 'tcx:'a'd:
../src/librustc/middle/expr_use_visitor.rs:279     pub fn new(delegate: &'d mut (Delegate<'tcx>),
../src/librustc/middle/expr_use_visitor.rs:280                typer: &'t infer::InferCtxt<'a, 'tcx>)
../src/librustc/middle/expr_use_visitor.rs:281                -> ExprUseVisitor<'d,'t,'a,'tcx> where 'tcx:'a+'d
../src/librustc/middle/expr_use_visitor.rs:282     {
../src/librustc/middle/expr_use_visitor.rs:283         let mc: mc::MemCategorizationContext<'t, 'a, 'tcx> =
../src/librustc/middle/expr_use_visitor.rs:284             mc::MemCategorizationContext::new(typer);
                                               ...
error: aborting due to previous error
make: *** [x86_64-apple-darwin/stage0/lib/rustlib/x86_64-apple-darwin/lib/stamp.rustc] Error 101
