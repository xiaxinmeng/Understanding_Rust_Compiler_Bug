plain
    Checking clippy_lints v0.1.70 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/redundant_async_block.rs:45:32
     |
45   |         if let ExprKind::Async(_, _, block) = &expr.kind && block.stmts.len() == 1 &&
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1431:11
     |
     |
1431 |     Async(CaptureBy, P<Block>),

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints` due to previous error
warning: build failed, waiting for other jobs to finish...
