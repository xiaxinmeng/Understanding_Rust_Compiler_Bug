plain
    Checking cargo_metadata v0.12.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.56 (/checkout/src/tools/clippy/clippy_lints)
error: `let` expressions are not supported here
    |
    |
120 |     if let ast::ExprKind::Block(ref block, _) = else_.kind
    |
    |
    = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: `let` expressions are not supported here
    |
    |
122 |         && let Some(else_) = expr_block(block)
    |
    |
    = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: `let` expressions are not supported here
    |
    |
125 |         && let ast::ExprKind::If(..) = else_.kind
    |
    |
    = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: `let` expressions are not supported here
    |
    |
142 |       && let Some(inner) = expr_block(then)
    |
    |
    = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: `let` expressions are not supported here
    |
    |
144 |       && let ast::ExprKind::If(ref check_inner, ref content, None) = inner.kind
    |
    |
    = note: only supported directly without parentheses in conditions of `if`- and `while`-expressions, as well as in `let` chains within parentheses

error: the feature `let_chains` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
11 | #![feature(let_chains)]
   |
   |
   = note: `-D incomplete-features` implied by `-D warnings`

error: could not compile `clippy_lints` due to 6 previous errors
Build completed unsuccessfully in 0:04:11
