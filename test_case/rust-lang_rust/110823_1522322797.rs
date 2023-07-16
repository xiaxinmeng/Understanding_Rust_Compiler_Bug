plain
   Compiling tokio v1.8.4
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:146:35
     |
146  |         (Try(l), Try(r)) | (Await(l), Await(r)) => eq_expr(l, r),
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1434:11
     |
     |
1434 |     Await(P<Expr>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
146  |         (Try(l), Try(r)) | (Await(l, _), Await(r)) => eq_expr(l, r),

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_utils/src/ast_utils.rs:146:45
     |
     |
146  |         (Try(l), Try(r)) | (Await(l), Await(r)) => eq_expr(l, r),
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1434:11
     |
     |
1434 |     Await(P<Expr>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
146  |         (Try(l), Try(r)) | (Await(l), Await(r, _)) => eq_expr(l, r),

   Compiling quote v1.0.26
   Compiling heck v0.4.0
    Checking strsim v0.10.0
