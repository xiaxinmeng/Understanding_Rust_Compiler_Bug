plain
    Checking url v2.1.1
    Checking toml v0.5.7
    Checking cargo_metadata v0.12.0
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   --> src/tools/clippy/clippy_lints/src/utils/higher.rs:246:50
    |
246 |                     if let ExprKind::Unary(UnOp::UnNot, condition) = clause.kind;
    |                                                  ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
    |
    |
115 |         if let ExprKind::Unary(UnOp::UnNot, ref expr) = cond.kind;
    |                                      ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   --> src/tools/clippy/clippy_lints/src/booleans.rs:113:39
    |
113 |                 ExprKind::Unary(UnOp::UnNot, inner) => return Ok(Bool::Not(box self.run(inner)?)),
    |                                       ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   --> src/tools/clippy/clippy_lints/src/booleans.rs:457:35
    |
457 |             ExprKind::Unary(UnOp::UnNot, inner) => {
    |                                   ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   --> src/tools/clippy/clippy_lints/src/booleans.rs:485:38
    |
485 |         if let ExprKind::Unary(UnOp::UnNot, inner) = &expr.kind {
    |                                      ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
  --> src/tools/clippy/clippy_lints/src/entry.rs:58:42
   |
58 |             if let ExprKind::Unary(UnOp::UnNot, ref check) = check.kind {
   |                                          ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:4068:29
     |
4068 |         if op == hir::UnOp::UnNot;
     |                             ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   --> src/tools/clippy/clippy_lints/src/needless_bool.rs:198:34
    |
198 |     if let ExprKind::Unary(UnOp::UnNot, operand) = e.kind {
    |                                  ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
   |
   |
53 |             if let ExprKind::Unary(UnOp::UnNot, ref inner) = expr.kind;
   |                                          ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNeg` found for enum `rustc_hir::UnOp` in the current scope
     |
     |
1709 |     matches!(expr.kind, ExprKind::Unary(UnOp::UnNeg, _))
     |                                               ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNeg` found for enum `rustc_hir::UnOp` in the current scope
     |
     |
1715 |         ExprKind::Unary(UnOp::UnNeg, e) => {
     |                               ^^^^^ variant or associated item not found in `rustc_hir::UnOp`

error[E0599]: no variant or associated item named `UnNot` found for enum `rustc_hir::UnOp` in the current scope
    |
    |
111 |     } else if let ExprKind::Unary(UnOp::UnNot, expr) = &expr.kind {
    |                                         ^^^^^ variant or associated item not found in `rustc_hir::UnOp`
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0599`.
error: could not compile `clippy_lints`
