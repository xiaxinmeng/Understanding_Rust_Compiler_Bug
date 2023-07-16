plain
    Checking clippy_lints v0.1.64 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/double_parens.rs:64:34
     |
64   |             ExprKind::MethodCall(_, ref params, _) => {
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
64   |             ExprKind::MethodCall(_, ref params, _, _) => {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/option_env_unwrap.rs:40:41
     |
     |
40   |             if let ExprKind::MethodCall(path_segment, args, _) = &expr.kind;
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
40   |             if let ExprKind::MethodCall(path_segment, args, _, _) = &expr.kind;

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/precedence.rs:112:44
     |
     |
112  |             while let ExprKind::MethodCall(path_segment, args, _) = &arg.kind {
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
112  |             while let ExprKind::MethodCall(path_segment, args, _, _) = &arg.kind {

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:598:23
     |
     |
598  |         | (MethodCall(_, _, _), MethodCall(_, _, _))
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
598  |         | (MethodCall(_, _, _, _), MethodCall(_, _, _))
help: use `..` to ignore all fields
     |
     |
598  |         | (MethodCall(..), MethodCall(_, _, _))

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/suspicious_operation_groupings.rs:598:44
     |
     |
598  |         | (MethodCall(_, _, _), MethodCall(_, _, _))
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
598  |         | (MethodCall(_, _, _), MethodCall(_, _, _, _))
help: use `..` to ignore all fields
     |
     |
598  |         | (MethodCall(_, _, _), MethodCall(..))

error[E0023]: this pattern has 3 fields, but the corresponding tuple variant has 4 fields
    --> src/tools/clippy/clippy_lints/src/unused_rounding.rs:33:33
     |
     |
33   |     if let ExprKind::MethodCall(name_ident, args, _) = &expr.kind
     |
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1347:16
     |
     |
1347 |     MethodCall(PathSegment, P<Expr>, Vec<P<Expr>>, Span),
     |
help: use `_` to explicitly ignore each field
     |
     |
33   |     if let ExprKind::MethodCall(name_ident, args, _, _) = &expr.kind

For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints` due to 6 previous errors
Build completed unsuccessfully in 0:03:57
