plain
    Checking url v2.2.2
    Checking semver v0.11.0
    Checking clippy_utils v0.1.56 (/checkout/src/tools/clippy/clippy_utils)
    Checking toml v0.5.7
error[E0422]: cannot find struct, variant or union type `Block` in this scope
   --> src/tools/clippy/clippy_utils/src/higher.rs:316:36
    |
316 |             if let ExprKind::Block(Block { stmts: [], expr: Some(expr), .. }, _) = arm.body.kind;
    |
help: consider importing one of these items
    |
6   | use crate::Block;
---

error[E0609]: no field `kind` on type `&_`
   --> src/tools/clippy/clippy_utils/src/higher.rs:317:56
    |
317 |             if let ExprKind::Call(_, call_args) = expr.kind;

Some errors have detailed explanations: E0422, E0609.
For more information about an error, try `rustc --explain E0422`.
error: could not compile `clippy_utils` due to 2 previous errors
