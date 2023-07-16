plain
[00:08:36]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:08:55] error[E0061]: this function takes 1 parameter but 2 parameters were supplied
[00:08:55]     --> libsyntax/parse/parser.rs:2808:51
[00:08:55]      |
[00:08:55] 2808 |                 let blk_expr = self.mk_expr(span, ExprKind::Block(blk, None), ThinVec::new());
[00:08:55]      | 
[00:08:55]      | 
[00:08:55]     ::: libsyntax/ast.rs:1128:5
[00:08:55]      |
[00:08:55] 1128 |     Block(P<Block>),
[00:08:55]      |     --------------- defined here
ry/Logs/DiagnosticReports/: No such file or directory
travis_time:end:25e44c20:start=1527365367052361486,finish=1527365367060050224,duration=7688738
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
