plain
    Checking thiserror v1.0.33
error[E0023]: this pattern has 2 fields, but the corresponding tuple variant has 3 fields
    --> src/tools/rustfmt/src/expr.rs:662:29
     |
662  |         ast::ExprKind::Loop(ref block, label) => {
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1364:10
     |
     |
1364 |     Loop(P<Block>, Option<Label>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
662  |         ast::ExprKind::Loop(ref block, label, _) => {

For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
