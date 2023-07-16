plain
[00:15:55]    Compiling rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
[00:16:02]    Compiling rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
[00:16:02]    Compiling rustc_passes v0.0.0 (/checkout/src/librustc_passes)
[00:16:24]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:16:24] error: incorrect close delimiter: `}`
[00:16:24]     |
[00:16:24]     |
[00:16:24] 280 |         if let ast::ExprKind::Paren(ref inner) = value.node {
[00:16:24]     |                                                             - close delimiter possibly meant for this
[00:16:24] 281 |             if struct_lit_needs_parens &&
[00:16:24] 282 |                     (parser::contains_exterior_struct_lit(&inner) ||
[00:16:24]     |                     - un-closed delimiter
[00:16:24] 323 |         }
[00:16:24]     |         ^ incorrect close delimiter
[00:16:24] 
[00:16:24] 
[00:16:24] error: expected one of `)`, `,`, `.`, `?`, or an operator, found `{`
[00:16:24]     |
[00:16:24]     |
[00:16:24] 283 |                     is_break(inner) {
[00:16:24]     |                                     ^ expected one of `)`, `,`, `.`, `?`, or an operator here
[00:16:25]    Compiling rustc_borrowck v0.0.0 (/checkout/src/librustc_borrowck)
[00:16:25] error: unused import: `syntax::errors::Applicability`
[00:16:25]   --> librustc_lint/unused.rs:20:5
[00:16:25]    |
---
[00:16:25]    Compiling rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
[00:16:26] error[E0308]: mismatched types
[00:16:26]    --> librustc_lint/unused.rs:268:16
[00:16:26]     |
[00:16:26] 268 |         if let ast::ExprKind::Break(..) = e {
[00:16:26]     |                ^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `syntax::ast::Expr`, found enum `syntax::ast::ExprKind`
[00:16:26]     = note: expected type `syntax::ast::Expr`
[00:16:26]                found type `syntax::ast::ExprKind`
[00:16:26] 
[00:16:26] error: aborting due to 6 previous errors
