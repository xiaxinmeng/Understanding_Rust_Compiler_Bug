plain
    Checking bstr v0.2.17
    Checking parking_lot v0.12.1
    Checking walkdir v2.3.2
   Compiling libz-sys v1.1.3
error[E0004]: non-exhaustive patterns: `rustc_ast::ExprKind::IncludedBytes(_, _, _)` not covered
    --> src/tools/clippy/clippy_utils/src/sugg.rs:190:15
190  |         match expr.kind {
190  |         match expr.kind {
     |               ^^^^^^^^^ pattern `rustc_ast::ExprKind::IncludedBytes(_, _, _)` not covered
note: `rustc_ast::ExprKind` defined here
    --> /checkout/compiler/rustc_ast/src/ast.rs:1450:5
     |
1305 | pub enum ExprKind {
1305 | pub enum ExprKind {
     | -----------------
...
1450 |     IncludedBytes(Span, std::path::PathBuf, Lrc<[u8]>),
     = note: the matched value is of type `rustc_ast::ExprKind`
     = note: the matched value is of type `rustc_ast::ExprKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
265  ~             ),
265  ~             ),
266  ~             rustc_ast::ExprKind::IncludedBytes(_, _, _) => todo!(),

    Checking idna v0.2.0
For more information about this error, try `rustc --explain E0004`.
error: could not compile `clippy_utils` due to previous error
